use std::cmp::Ordering;


fn calculate_fuel (mass:i64) -> i64 {
    let fuel:i64 = (mass / 3) - 2; 

    println!("{}, {}",mass, fuel);
   match fuel.cmp(&0){
     Ordering::Less | Ordering::Equal=> 0,
     Ordering::Greater => calculate_fuel(fuel) + fuel
    }


}


fn run() {
let masses:[ i64;100 ] = [
    77680,
106826,
120225,
122031,
100287,
70402,
145496,
73956,
148205,
52065,
149237,
116195,
84309,
105327,
134606,
109489,
104474,
69536,
141469,
72321,
75154,
142565,
57104,
111925,
100109,
75178,
115130,
75586,
148587,
116029,
113969,
66072,
90521,
116324,
137009,
92880,
110895,
131046,
83273,
99576,
70466,
93602,
63435,
103569,
56610,
58392,
95060,
59101,
121838,
93494,
52243,
146982,
142260,
107232,
117600,
59715,
80284,
128223,
123676,
81060,
99425,
50621,
101184,
112136,
131262,
53150,
113522,
117802,
120251,
102322,
111247,
117719,
88873,
133541,
92695,
125445,
149048,
146928,
83909,
109466,
94063,
62338,
124098,
64769,
104722,
106612,
53882,
108847,
92739,
88496,
89773,
57471,
140532,
87308,
137247,
62816,
118893,
101446,
149208,
68267,
];

let fuel:i64 = masses.iter().map(|mass| calculate_fuel(*mass) ).sum();
println!("{:?}", fuel);

}
