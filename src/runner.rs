use std::{io::{Write, Read}, num::Wrapping, net::{TcpStream, SocketAddr}};
use crate::parser::{parse, Instruction, OpCode, ParseError};

pub struct Environment {
    tape: Vec<Wrapping<i32>>,
    connections: Vec<TcpStream>,
    socket_handle_counter: u16,
    handle: Option<usize>,
    port: Option<u16>,
    ptr: usize,
}

impl Environment {
    pub fn new(tape_size: usize) -> Self {
        Environment {
            tape: vec![Wrapping(0); tape_size],
            socket_handle_counter: 0,
            connections: vec![],
            handle: None,
            port: None,
            ptr: 0,
        }
    }

    fn execute(&mut self, instructions: &[Instruction]) {
        for instruction in instructions {
            match instruction.opcode {
                OpCode::Left => {
                    self.ptr -= instruction.amount;
                }
                OpCode::Right => {
                    self.ptr += instruction.amount;
                }
                OpCode::Inc => self.tape[self.ptr] += instruction.amount as i32,
                OpCode::Dec => self.tape[self.ptr] -= instruction.amount as i32,
                OpCode::Loop => {
                    while self.tape[self.ptr] != Wrapping(0) {
                        self.execute(&instruction.instructions)
                    }
                }
                OpCode::Query => {
                    for _ in 0..instruction.amount {
                        let char = console::Term::stdout()
                            .read_char()
                            .expect("Terminal is not user attended");
                        self.tape[self.ptr] = Wrapping(char as i32);
                    }
                }
                OpCode::Print => {
                    let char = ((self.tape[self.ptr].0 % 256) as u8 as char).to_string();
                    print!("{}", char.repeat(instruction.amount));
                    std::io::stdout().flush().unwrap();
                }

                // networking
                OpCode::SetPort => {
                    let current_value = self.tape[self.ptr].0;
                    if current_value < 0 {
                        self.tape[self.ptr] = Wrapping(-1);
                    } else {
                        self.port = Some((current_value % 65535) as u16);
                        println!("port set -- {}", current_value);
                    }
                }

                OpCode::SocketHandle => {
                    let current_value = self.tape[self.ptr].0 as usize;
                    if self.connections.get(current_value).is_none() {
                        self.tape[self.ptr] = Wrapping(-1);
                    } else {
                        self.handle = Some(current_value);
                        println!("handle set!");
                    }
                }

                OpCode::Connect => {
                    if self.port.is_none() {
                        self.tape[self.ptr] = Wrapping(-1);
                        println!("no port");
                        continue;
                    }

                    let current_value = self.tape[self.ptr].0;
                    let addr = SocketAddr::from((current_value.to_be_bytes(), self.port.unwrap()));
                    let connection = TcpStream::connect(addr).ok();

                    if let Some(connection) = connection {
                        self.connections.push(connection);
                        self.tape[self.ptr] = Wrapping(self.socket_handle_counter.into());
                        self.socket_handle_counter += 1;
                        println!("connection succeded!");
                    } else {
                        self.tape[self.ptr] = Wrapping(-1);
                        println!("failed to connect");
                    }
                }

                OpCode::Disconnect => {
                    if self.handle.is_none() || self.connections.get(self.handle.unwrap()).is_none() {
                        self.tape[self.ptr] = Wrapping(-1);
                        continue;
                    }

                    let connection = self.connections.get(self.handle.unwrap()).unwrap();
                    if connection.shutdown(std::net::Shutdown::Both).is_err() {
                        self.tape[self.ptr] = Wrapping(-1);
                        println!("closed!");
                    }
                }

                OpCode::SendData => {
                    if self.handle.is_none() || self.connections.get(self.handle.unwrap()).is_none() {
                        self.tape[self.ptr] = Wrapping(-1);
                        continue;
                    }

                    let mut connection = self.connections.get(self.handle.unwrap()).unwrap();
                    if instruction.amount > 4 {
                        for _ in 0..instruction.amount {
                            let current_value = self.tape[self.ptr].0;
                            let bytes_to_send = &[current_value.to_le_bytes()[0]];
                            if connection.write_all(bytes_to_send).is_err() { // [0] for LSB?
                                                                                   // i think that's wrong but
                                    self.tape[self.ptr] = Wrapping(-1);
                            }
                        }
                    } else {
                        let current_value = self.tape[self.ptr].0;
                        let bytes = current_value.to_le_bytes();
                        let bytes_to_send = &bytes[0..instruction.amount];
                        if connection.write_all(bytes_to_send).is_err() { // this also might be wrong
                                                                               // endian is confusing...
                                self.tape[self.ptr] = Wrapping(-1);
                        }
                    }
                }

                OpCode::FlushWrites => {
                    if self.handle.is_none() || self.connections.get(self.handle.unwrap()).is_none() {
                        self.tape[self.ptr] = Wrapping(-1);
                        continue;
                    }
                    
                    let mut connection = self.connections.get(self.handle.unwrap()).unwrap();
                    if connection.flush().is_err() {
                        self.tape[self.ptr] = Wrapping(-1);
                    }
                }

                OpCode::RecieveData => {
                    if self.handle.is_none() || self.connections.get(self.handle.unwrap()).is_none() {
                        self.tape[self.ptr] = Wrapping(-1);
                        continue;
                    }

                    let mut connection = self.connections.get(self.handle.unwrap()).unwrap();
                    let mut buffer = vec![0; instruction.amount];
                    let read_result = connection.read_exact(&mut buffer);

                    if read_result.is_err() {
                        self.tape[self.ptr] = Wrapping(-1);
                    } else {
                        if buffer.len() < 4 {
                            buffer.extend(std::iter::repeat(0).take(4 - buffer.len()));
                        }

                        let last_four = buffer.iter().rev().take(4).rev().map(|&x| x).collect::<Vec<u8>>();
                        let last_four_slice = last_four.as_slice();

                        println!("{:?}", last_four);

                        self.tape[self.ptr] = Wrapping(i32::from_le_bytes(last_four_slice.try_into().unwrap()));
                    }
                }
            }
        }
    }

    pub fn evaluate(&mut self, input: &str) -> Result<(), ParseError> {
        let instructions = parse(input)?;
        self.execute(&instructions);

        return Ok(());
    }
}
