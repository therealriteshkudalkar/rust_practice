use std::collections::{HashMap, HashSet};

fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
    // Create an Adjacency List in a HashMap using prerequisites
    let mut dependency_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    for i in 0..num_courses {
        dependency_map.insert(i, HashSet::new());
    }

    // Loop through the edges and form a dependency map
    for prerequisite in prerequisites {
        let course_a = prerequisite[0];
        let course_b = prerequisite[1];
        dependency_map.get_mut(&course_a).unwrap().insert(course_b);
    }

    // Perform Topological Sort
    loop {
        // Check all the nodes which have
        let mut course_without_prerequisites: HashSet<i32> = HashSet::new();
        for (course, course_prerequisites) in &dependency_map {
            if course_prerequisites.len() == 0 {
                course_without_prerequisites.insert(*course);
            }
        }

        if course_without_prerequisites.is_empty() {
            break;
        }

        // Remove the nodes which have the dependency array
        for course in &course_without_prerequisites {
            dependency_map.remove(course);
        }

        let mut remaining_courses: Vec<i32> = Vec::new();
        for courses in dependency_map.keys() {
            remaining_courses.push(*courses);
        }

        // Remove the courses without prerequisites from the dependency list of other
        for course in remaining_courses {
            let difference: HashSet<i32> = dependency_map.get(&course).unwrap()
                .difference(&course_without_prerequisites).cloned().collect();
            dependency_map.insert(course, difference);
        }
    }

    return dependency_map.is_empty();
}

pub fn main45() {
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0]];
    println!("Number of courses: {num_courses}; Pre-requisites: {:?}; Can Finish: {}",
             prerequisites, can_finish(num_courses, prerequisites.clone()));

    let num_courses = 2;
    let prerequisites = vec![vec![1, 0], vec![0, 1]];
    println!("Number of courses: {num_courses}; Pre-requisites: {:?}; Can Finish: {}",
             prerequisites, can_finish(num_courses, prerequisites.clone()));
}