pub fn execute() {
    let result = prims::PrimsGraph::new(vec![
        ("a", "f", 2),
        ("a", "b", 2),
        ("a", "d", 7),
        ("f", "b", 5),
        ("b", "d", 4),
        ("f", "c", 4),
        ("b", "c", 1),
        ("b", "e", 3),
        ("c", "e", 4),
        ("d", "e", 1),
        ("e", "g", 7),
        ("d", "g", 5),
    ]);
}

mod prims {
    use crate::graph_paths::priority_queue;
    use rand::Rng;
    use std::cmp::Eq;
    use std::collections::HashMap;
    use std::fmt::{Debug, Display};
    use std::hash::Hash;

    #[derive(Clone, Debug)]
    pub struct PrimsVertex<T> {
        vertex: T,
        distance_from_parent: f64,
        parent: Option<T>,
        in_mst: bool,
    }

    pub type GraphType<T> = HashMap<T, Vec<(T, u64)>>;

    #[derive(Clone, Debug)]
    pub struct PrimsGraph<T> {
        graph: Option<GraphType<T>>,
        all_edges: Vec<Edge<T>>,
    }

    #[derive(Clone, Debug)]
    pub struct MST<T> {
        start_vertex: T,
        end_vertex: T,
    }

    #[derive(Copy, Clone, Debug)]
    pub struct Edge<T> {
        first: T,
        second: T,
        weight: u64,
    }
    impl<T> PrimsGraph<T>
    where
        T: Debug + Eq + Hash + Copy + Display + Clone,
    {
        pub fn new(graph_info: Vec<(T, T, u64)>) {
            let mut adjacency_dic: HashMap<T, Vec<(T, u64)>> = HashMap::new();

            let mut dic_distance: HashMap<T, PrimsVertex<T>> = HashMap::new();

            let mut all_vertex: Vec<T> = vec![];
            let mut all_edges: Vec<Edge<T>> = vec![];

            for value in graph_info {
                {
                    let vertex_x = &mut adjacency_dic.entry(value.0).or_insert(vec![]);
                    vertex_x.push((value.1, value.2));
                }
                let vertex_y = &mut adjacency_dic.entry(value.1).or_insert(vec![]);
                vertex_y.push((value.0, value.2));

                all_vertex.push(value.0);
                all_vertex.push(value.1);

                all_edges.push(Edge {
                    first: value.0,
                    second: value.1,
                    weight: value.2,
                });

                dic_distance.insert(
                    value.0,
                    PrimsVertex {
                        vertex: value.0,
                        parent: None,
                        distance_from_parent: f64::INFINITY,
                        in_mst: false,
                    },
                );

                dic_distance.insert(
                    value.1,
                    PrimsVertex {
                        vertex: value.1,
                        parent: None,
                        distance_from_parent: f64::INFINITY,
                        in_mst: false,
                    },
                );
            }

            let the_graph = PrimsGraph {
                graph: Some(adjacency_dic),
                all_edges,
            };

            let mut rng = rand::thread_rng();
            let n: usize = rng.gen_range(0, all_vertex.len());
            let start_vertex = all_vertex.get(n).unwrap();

            the_graph.get_mst(&mut dic_distance, *start_vertex);
        }

        pub fn get_mst(
            &self,
            dic_distance: &mut HashMap<T, PrimsVertex<T>>,
            start_vertex: T,
        ) -> Vec<MST<T>> {
            let mut mst_tree: Vec<MST<T>> = vec![];

            let mut priority_queue = priority_queue::BinaryHeap::new(20);
            priority_queue.insert((0, start_vertex));

            while priority_queue.total_length() > 0 {
                let v = priority_queue.extract_min().unwrap();

                // {
                // for the sake of the mutable borrow use scopes so we can deref after this block
                // let the_vertex = dic_distance.get_mut(&v.node).unwrap();
                // println!(
                //     "after the unwrap THE VERTEX ===>>> {:?}  +++++++++++{:?}",
                //     the_vertex.vertex, the_vertex.parent
                // );

                // println!(
                //     "HOW TRUE IS THIS??????? {:?}",
                //     dic_distance.get_mut(&v.node).unwrap().in_mst
                // );
                println!(
                    "WE HAVE TO PRINT THIS PLS!!!!!!>>>>>>>>>>????????? {:#?}",
                    dic_distance.get_mut(&v.node).unwrap()
                );

                // println!(
                //     "TH CONDITIONS ONE {:#?}",
                //     dic_distance.get_mut(&v.node).unwrap().parent.is_some()
                // );
                // println!(
                //     "THE CONDITION TWO {:#?}",
                //     !dic_distance.get_mut(&v.node).unwrap().in_mst
                // );
                if dic_distance.get_mut(&v.node).unwrap().parent.is_some()
                    && !dic_distance.get_mut(&v.node).unwrap().in_mst
                {
                    println!("TESTED AND TRSUSTED !!!!!");
                    mst_tree.push(MST {
                        start_vertex: dic_distance.get_mut(&v.node).unwrap().vertex,
                        end_vertex: dic_distance.get_mut(&v.node).unwrap().parent.unwrap(),
                    });
                }
                dic_distance.get_mut(&v.node).unwrap().in_mst = true;
                // }

                // println!("AT THIS POTINT {:#?}", self.graph);

                let all_neighbours = self
                    .graph
                    .as_ref()
                    .unwrap()
                    .get(&dic_distance.get(&v.node).unwrap().vertex)
                    .unwrap();

                for value in all_neighbours {
                    let current_neighbour = dic_distance.get(&value.0).unwrap();
                    let current_edge = self
                        .all_edges
                        .iter()
                        .find(|&&x| {
                            (x.first == value.0
                                || x.first == dic_distance.get(&v.node).unwrap().vertex)
                                && (x.second == value.0
                                    || x.second == dic_distance.get(&v.node).unwrap().vertex)
                        })
                        .unwrap();

                    if !dic_distance.get(&value.0).unwrap().in_mst
                        && current_neighbour.distance_from_parent > current_edge.weight as f64
                    {
                        println!("MADE IT THIS FAR {:?}", value);
                        let the_vertex = dic_distance.get_mut(&v.node).unwrap();
                        if the_vertex.vertex == current_edge.first {
                            // println!("ENTERED HERE THE FIRST ********* {:?}", current_edge.first);
                            the_vertex.parent = Some(current_edge.second);
                            the_vertex.distance_from_parent = current_edge.weight as f64;
                            priority_queue.insert((current_edge.weight as i8, current_edge.second));
                        // println!(
                        //     "WE HAVE TO PRINT THIS PLS--------------------------------------- {:#?}---------------------------------------",
                        //     dic_distance
                        // );
                        } else {
                            // println!("ENTERED THE SECOND ########## {:?}", current_edge.second);
                            the_vertex.parent = Some(current_edge.first);
                            the_vertex.distance_from_parent = current_edge.weight as f64;
                            priority_queue.insert((current_edge.weight as i8, current_edge.first));
                            // println!(
                            //     "--------------------------------------- {:#?}---------------------------------------",
                            //     dic_distance
                            // );
                        }
                    }
                }
            }

            // println!(
            //     "WE WERE DOWN HERE FOR THE dic_distance>>>>>>>>>. {:#?} <<<<<<<<<<<<<<<<",
            //     dic_distance
            // );

            println!("WE WERE DOWN HERE FOR THE MST {:#?}", mst_tree);

            return mst_tree;
        }
    }
}
