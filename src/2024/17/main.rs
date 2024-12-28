use std::collections::VecDeque;

#[derive(Debug, Default, Clone, Hash)]
struct Machine {
    ra: u64,
    rb: u64,
    rc: u64,
    ip: usize,
    program: Vec<u64>,
    output: Vec<u64>,
}

impl Machine {
    fn run_once(&mut self) {
        let opcode = self.program[self.ip];
        let literal = self.program[self.ip + 1];
        let combo = match self.program[self.ip + 1] {
            0..=3 => literal,
            4 => self.ra,
            5 => self.rb,
            6 => self.rc,
            7 => 0,
            _ => unreachable!(),
        };

        match opcode {
            0 => self.ra = self.ra >> combo,  // adv
            1 => self.rb = self.rb ^ literal, // bxl
            2 => self.rb = combo % 8,         // bst
            3 => {
                // jnz
                if self.ra != 0 {
                    self.ip = literal as usize;
                }
            }
            4 => self.rb = self.rb ^ self.rc, // bxc
            5 => self.output.push(combo % 8), // out
            6 => self.rb = self.ra >> combo,  // bdv
            7 => self.rc = self.ra >> combo,  // cdv
            _ => unreachable!(),
        }

        if !(opcode == 3 && self.ra != 0) {
            self.ip += 2;
        }
    }

    fn run(&mut self) {
        while self.ip + 1 < self.program.len() {
            self.run_once();
        }
    }
}

fn part_one(machine: &Machine) {
    let mut machine = machine.clone();
    machine.run();

    let output: Vec<_> = machine.output.iter().map(|num| num.to_string()).collect();
    let output = output.join(",");

    println!("{}", output);
}

fn part_two(machine: &Machine) {
    // RANT: Big thanks to the rise of AI for making this year of AoC the worst it's ever been!

    // I spent 2 days trying to find a generalized solution to this problem (assuming the input is
    // always solvable), and such a solution is simply not possible (or would require hundreds of
    // edge case checking / automated "manual" assembly analysis with many assumptions made on the
    // input data, none of which are explained in detail by the problem statement).

    // Although good for keeping AI in check, this problem feels too "hacky"

    // I was looking forwards to returning to AoC, and finally pushing my solutions to VCS, but if
    // this continues, this might be the last year I ever do AoC.

    // My program:
    // (1) Rb = Ra % 8
    // (2) Rb = Rb ^ 2
    // (3) Rc = Ra >> Rb
    // (4) Rb = Rb ^ Rc
    // (5) Ra = Ra >> 3
    // (6) Rb = Rb ^ 7
    // (7) OUT, Rb % 8
    // (8) JNZ, 0

    let mut possibilities = VecDeque::new();
    possibilities.push_back((0, machine.program.len() - 1));

    while !possibilities.is_empty() {
        let (prev_ra, out_idx) = possibilities.pop_front().unwrap();

        for offset in 0..8 {
            let ra = (prev_ra << 3) + offset;
            let mut rb: u64;
            let rc: u64;

            rb = ra % 8;
            rb = rb ^ 2;
            rc = ra >> rb;
            rb = rb ^ rc;
            //ra = ra >> 3;
            rb = rb ^ 7;

            let out = rb % 8;
            if machine.program[out_idx] == out {
                if out_idx == 0 {
                    println!("{}", ra);
                    return;
                }

                possibilities.push_back((ra, out_idx - 1));
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 17)?;

    let (registers, program) = input.split_once("\n\n").unwrap();

    let mut registers = registers.lines().map(|line| {
        let register = line.split_once(": ").unwrap().1;
        let register = register.parse().unwrap();
        register
    });
    let ra = registers.next().unwrap();
    let rb = registers.next().unwrap();
    let rc = registers.next().unwrap();

    let program = program.split_once(": ").unwrap().1;
    let program = program.split(',').map(|num| num.parse().unwrap()).collect();

    let machine = Machine {
        ra,
        rb,
        rc,
        program,
        ..Default::default()
    };

    part_one(&machine);
    part_two(&machine);

    Ok(())
}
