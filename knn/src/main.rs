struct Point {
    features: Vec<f64>,
    label: String,
}

struct DataSet {
    data: Vec<Point>,
}

fn euclidean_distance(a: &Vec<f64>, b: &Vec<f64>) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2))
        .sum::<f64>()
        .sqrt()
}

impl DataSet {
    fn k_nearest_neighbors(&self, test_point: &Point, k: usize) -> Vec<&Point> {
        let mut distances = self
            .data
            .iter()
            .map(|p| (euclidean_distance(&p.features, &test_point.features), p))
            .collect::<Vec<_>>();

        distances.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        distances.into_iter().take(k).map(|(_, p)| p).collect()
    }

    fn predict(&self, test_point: &Point, k: usize) -> String {
        let neighbors = self.k_nearest_neighbors(test_point, k);
        let mut counts = std::collections::HashMap::new();
        for neighbor in neighbors {
            *counts.entry(&neighbor.label).or_insert(0) += 1;
        }

        counts
            .into_iter()
            .max_by_key(|&(_, count)| count)
            .map(|(label, _)| label.clone())
            .unwrap_or_else(|| String::from("Unknown"))
    }
}

fn main() {
    // Initialize a dataset with sample data
    let data = DataSet {
        data: vec![
            Point { features: vec![1.0, 2.0], label: "A".to_string() },
            Point { features: vec![2.0, 3.0], label: "A".to_string() },
            Point { features: vec![5.0, 5.0], label: "B".to_string() },
            Point { features: vec![6.0, 5.0], label: "B".to_string() },
            Point { features: vec![1.0, 0.5], label: "A".to_string() },
            Point { features: vec![6.0, 5.5], label: "B".to_string() },
        ],
    };

    // Create multiple test instances
    let test_points = vec![
        Point { features: vec![0.0, 1.5], label: "A".to_string() }, // True label "A"
        Point { features: vec![6.0, 5.0], label: "B".to_string() }, // True label "B"
        Point { features: vec![1.0, 2.0], label: "A".to_string() }, // True label "A"
        Point { features: vec![5.0, 4.0], label: "B".to_string() }, // True label "B"
    ];

    // Define the number of neighbors
    let k = 3;

    // Evaluate each test instance
    for (i, test_point) in test_points.iter().enumerate() {
        let predicted_label = data.predict(test_point, k);
        println!("Test Point {}: Features = {:?}, True Label = {}, Predicted Label = {}",
                 i + 1, test_point.features, test_point.label, predicted_label);
    }
}

