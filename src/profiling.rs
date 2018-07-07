// use flame::*;
// use std::cmp::Ordering;
// use std::collections::BTreeMap;

// #[derive(Debug)]
// pub struct Profile {
//     spans: BTreeMap<StrCow, (u64, u64)>,
//     total_time: u64,
// }

// #[derive(Debug, Eq)]
// pub struct SpanTotal {
//     name: StrCow,
//     time: u64,
//     calls: u64,
// }

// impl Ord for SpanTotal {
//     fn cmp(&self, other: &SpanTotal) -> Ordering {
//         self.time.cmp(&other.time)
//     }
// }

// impl PartialOrd for SpanTotal {
//     fn partial_cmp(&self, other: &SpanTotal) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl PartialEq for SpanTotal {
//     fn eq(&self, other: &SpanTotal) -> bool {
//         self.name == other.name
//     }
// }

// impl Profile {
//     pub fn from_spans(spans: Vec<Span>) -> Profile {
//         println!("Summing span information...");
//         // create an empty dictionary/map to total the spans
//         let mut spans_map: BTreeMap<StrCow, (u64, u64)> = BTreeMap::new();

//         // iterate recursively over the spans, adding them to the spans
//         fn add_span(s: Span, spans_map: &mut BTreeMap<StrCow, (u64, u64)>) -> () {
//             // update the map with this span
//             spans_map
//                 .entry(s.name.clone())
//                 .and_modify(|p| {
//                     let (t, c) = *p;
//                     *p = (t + s.delta, c + 1);
//                 })
//                 .or_insert((s.delta, 1));

//             // recurse into the sub-entries and add them to the map
//             for c in s.children {
//                 add_span(c, spans_map);
//             }
//         };

//         // iterate over the list of spans passed, and add them to the map
//         let total_time = spans[0].delta;

//         for s in spans {
//             add_span(s, &mut spans_map);
//         }

//         Profile {
//             spans: spans_map,
//             total_time: total_time,
//         }
//     }

//     fn seconds(ns: u64) -> f64 {
//         ns as f64 / 1000000000.0
//     }

//     pub fn print(self: &Self) -> () {
//         println!("Calculating profiling information...");
//         // collect the times as an array
//         let mut span_vec: Vec<SpanTotal> = self
//             .spans
//             .iter()
//             .map(|(k, (t, c))| SpanTotal {
//                 name: k.clone(),
//                 time: *t,
//                 calls: *c,
//             })
//             .collect();
//         span_vec.sort();

//         // calculate the totals
//         // let total_time : u64 = span_vec.iter().fold(0, |acc, s| acc + s.time);

//         // iterate over the spans, print the names, times, and percentages
//         println!(
//             "{0: <30} | {1: ^20.10} | {2: ^10.10} | {3: ^15.10} | {4: ^20.10}",
//             "Span name", "time (s)", "calls", "time/call (s)", "percentage"
//         );
//         for s in span_vec.iter() {
//             println!(
//                 "{0: <30} | {1: ^20.10} | {2: ^10.10} | {3: ^15.10} | {4: ^20.10}",
//                 s.name,
//                 Profile::seconds(s.time),
//                 s.calls,
//                 Profile::seconds(s.time) / s.calls as f64,
//                 100.0 * (s.time as f64 / self.total_time as f64)
//             );
//         }
//     }
// }
