use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Input {
    wire: String,
    value: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Gate {
    And,
    Or,
    Xor,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Wiring {
    gate: Gate,
    input1: String,
    input2: String,
    output: String,
}

fn create_wire_list(wirings: &Vec<Wiring>) -> Vec<String> {
    let mut wire_list = BTreeSet::new();

    for wiring in wirings.iter() {
        wire_list.insert(wiring.input1.clone());
        wire_list.insert(wiring.input2.clone());
        wire_list.insert(wiring.output.clone());
    }

    wire_list.into_iter().collect()
}

fn create_gate_list(
    wirings: &Vec<Wiring>,
    wire_list: &Vec<String>,
) -> Vec<Vec<(Gate, usize, usize)>> {
    let wires = wire_list.len();
    let mut gate_list = vec![Vec::new(); wires];

    for wiring in wirings.iter() {
        let gate = wiring.gate;
        let input1 = wire_list.binary_search(&wiring.input1).unwrap();
        let input2 = wire_list.binary_search(&wiring.input2).unwrap();
        let output = wire_list.binary_search(&wiring.output).unwrap();

        gate_list[input1].push((gate, input2, output));
        gate_list[input2].push((gate, input1, output));
    }

    gate_list
}

fn part_one(inputs: &Vec<Input>, wirings: &Vec<Wiring>) {
    let wire_list = create_wire_list(wirings);
    let gate_list = create_gate_list(wirings, &wire_list);
    let wires = wire_list.len();

    let mut wire_values = vec![None; wires];
    let mut wires_to_process = vec![];

    for input in inputs.iter() {
        if let Ok(wire) = wire_list.binary_search(&input.wire) {
            wires_to_process.push((wire, input.value));
        }
    }

    while !wires_to_process.is_empty() {
        let (wire, value) = wires_to_process.pop().unwrap();

        if wire_values[wire].is_some() {
            panic!("conflict wtf (outputs should only be set once)");
        }
        wire_values[wire] = Some(value);

        for &(gate, other, output) in gate_list[wire].iter() {
            if let Some(other_value) = wire_values[other] {
                let result = match gate {
                    Gate::And => value & other_value,
                    Gate::Or => value | other_value,
                    Gate::Xor => value ^ other_value,
                };
                wires_to_process.push((output, result));
            }
        }
    }

    let mut num = 0;

    for (wire, value) in wire_values.iter().enumerate() {
        let wire = &wire_list[wire];
        let value = value.expect("z wire has value");

        if wire.starts_with('z') {
            let bit: u64 = wire.trim_start_matches('z').parse().unwrap();

            num |= value << bit;
        }
    }

    println!("{}", num);
}

fn part_two(_inputs: &Vec<Input>, wirings: &Vec<Wiring>) {
    // Perform swaps first for manual analysis
    let mut wirings = wirings.clone();
    let n = wirings.len();

    let swaps = [
        ("qnw", "z15"),
        ("cqr", "z20"),
        ("vkg", "z37"),
        ("ncd", "nfj"), // at z27
    ];
    for swap in swaps {
        let mut i = 0;
        let mut j = 0;
        for k in 0..n {
            if wirings[k].output == swap.0 {
                i = k;
            }
            if wirings[k].output == swap.1 {
                j = k;
            }
        }

        let tmp = std::mem::take(&mut wirings[i].output);
        wirings[i].output = std::mem::take(&mut wirings[j].output);
        wirings[j].output = tmp;
    }

    let mut swaps: Vec<_> = swaps
        .into_iter()
        .map(|(s1, s2)| [s1, s2])
        .flatten()
        .collect();
    swaps.sort();
    let swaps = swaps.join(",");
    println!("\nPerformed swaps: {}", swaps);

    // Generate Graphviz DOT script to visualize the circuit, manually detect invalid patterns
    let mut x_inputs = BTreeSet::new();
    let mut y_inputs = BTreeSet::new();
    let mut z_outputs = BTreeSet::new();

    let mut and_outputs = BTreeSet::new();
    let mut or_outputs = BTreeSet::new();
    let mut xor_outputs = BTreeSet::new();

    // Assumption: x-- and y-- will never be used as outputs
    // Assumption: z-- will never be used as inputs

    for wiring in wirings.iter() {
        for input in [&wiring.input1, &wiring.input2] {
            if input.starts_with('x') {
                x_inputs.insert(input.clone());
            }
            if input.starts_with('y') {
                y_inputs.insert(input.clone());
            }
        }

        let output = wiring.output.clone();

        if output.starts_with('z') {
            z_outputs.insert(output.clone());
        }

        match wiring.gate {
            Gate::And => and_outputs.insert(output),
            Gate::Or => or_outputs.insert(output),
            Gate::Xor => xor_outputs.insert(output),
        };
    }

    let and_outputs = Vec::from_iter(and_outputs).join("; ");
    let and_outputs_styles = format!(
        r#"subgraph and_outputs {{
    node [style=filled, color=pink];
    {and_outputs};
}}"#
    );

    let or_outputs = Vec::from_iter(or_outputs).join("; ");
    let or_outputs_styles = format!(
        r#"subgraph or_outputs {{
    node [style=filled, color=lightblue];
    {or_outputs};
}}"#
    );

    let xor_outputs = Vec::from_iter(xor_outputs).join("; ");
    let xor_outputs_styles = format!(
        r#"subgraph xor_outputs {{
    node [style=filled, color=lightgreen];
    {xor_outputs};
}}"#
    );

    let x_inputs = Vec::from_iter(x_inputs).join(" -> ");
    let x_inputs_styles = format!(
        r#"subgraph x_inputs {{
    node [style=filled, color=lightgray];
    edge [style=invis];
    {x_inputs};
}}"#
    );

    let y_inputs = Vec::from_iter(y_inputs).join(" -> ");
    let y_inputs_styles = format!(
        r#"subgraph y_inputs {{
    node [style=filled, color=lightgray];
    edge [style=invis];
    {y_inputs};
}}"#
    );

    let z_outputs = Vec::from_iter(z_outputs).join(" -> ");
    let z_outputs_styles = format!(
        r#"subgraph z_outputs {{
    node [style=filled, color=lightgray];
    edge [style=invis];
    {z_outputs};
}}"#
    );

    let dot_edges: Vec<_> = wirings
        .iter()
        .map(|wiring| {
            format!(
                "{0} -> {2}; {1} -> {2};",
                wiring.input1, wiring.input2, wiring.output
            )
        })
        .collect();
    let dot_edges = dot_edges.join("\n");

    let dot_script = format!(
        r#"
digraph circuit {{
    {and_outputs_styles}
    {or_outputs_styles}
    {xor_outputs_styles}

    {x_inputs_styles}
    {y_inputs_styles}
    {z_outputs_styles}

    {dot_edges}
}}
"#
    );

    println!("{dot_script}");
}

fn main() -> std::io::Result<()> {
    let input = aoc::read_input(2024, 24)?;

    let (inputs, wirings) = input.split_once("\n\n").unwrap();
    let inputs: Vec<Input> = inputs
        .lines()
        .map(|line| {
            let (wire, value) = line.split_once(": ").unwrap();
            let wire = wire.to_owned();
            let value = value.parse().unwrap();
            Input { wire, value }
        })
        .collect();
    let wirings: Vec<Wiring> = wirings
        .lines()
        .map(|line| {
            let words: Vec<_> = line.split_whitespace().collect();
            let input1 = words[0];
            let gate = words[1];
            let input2 = words[2];
            let output = words[4];

            let gate = match gate {
                "AND" => Gate::And,
                "OR" => Gate::Or,
                "XOR" => Gate::Xor,
                _ => unreachable!(),
            };
            let input1 = input1.to_owned();
            let input2 = input2.to_owned();
            let output = output.to_owned();

            Wiring {
                gate,
                input1,
                input2,
                output,
            }
        })
        .collect();

    part_one(&inputs, &wirings);
    part_two(&inputs, &wirings);

    Ok(())
}
