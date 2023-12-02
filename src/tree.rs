use egui::Ui;

/// Represents a branch in the tree.
struct Branch {
    /// Current depth of the branch in the tree
    level: u32,
    /// Origin position of the branch
    origin: (f32, f32),
    /// Heading of the branch
    heading: f32,
    /// Length of the branch
    length: f32,
}

impl Branch {
    /// Calculate the end position of the branch.
    fn end(&self) -> (f32, f32) {
        (
            self.length.mul_add(self.heading.cos(), self.origin.0),
            self.length.mul_add(self.heading.sin(), self.origin.1),
        )
    }
}

type Branches = Vec<Branch>;

pub trait EguiDrawable {
    fn ui(&mut self, ui: &mut Ui);
}

#[derive(Clone, Copy, PartialEq)]
struct TreeOptions {
    levels: u32,
    base_width: f32,
    base_length: f32,
    angle_diff: f32,
    length_contraction: f32,
    width_contraction: f32,
    colored: bool,
}

impl TreeOptions {
    fn new() -> Self {
        Self {
            levels: 4,
            base_width: 1.0,
            base_length: 0.5,
            angle_diff: PI / 4.0,
            length_contraction: 0.5,
            width_contraction: 0.9,
            colored: true,
        }
    }
}

impl EguiDrawable for TreeOptions {
    fn ui(&mut self, ui: &mut Ui) {
        let ctx = ui.ctx();

        Window::new("Settings").show(ctx, |ui| {
            ui.add(Slider::new(&mut self.levels, 1..=20).text("Levels"));
            ui.add(Slider::new(&mut self.base_width, 0.0..=5.0).text("Base Width"));
            ui.add(Slider::new(&mut self.base_length, 0.0..=1.0).text("Base Length"));
            ui.add(Slider::new(&mut self.angle_diff, 0.0..=PI).text("Angle difference"));
            ui.add(Slider::new(&mut self.length_contraction, 0.0..=1.0).text("Length Contraction"));
            ui.add(Slider::new(&mut self.width_contraction, 0.0..=1.0).text("Width Contraction"));
        });
    }
}

pub struct Tree {
    options: TreeOptions,
    last_options: TreeOptions,
    branches: Branches,
}

impl Tree {
    pub fn new(levels: u32) -> Self {
        let mut options = TreeOptions::new();
        options.levels = levels;
        Self {
            options,
            last_options: options,
            branches: Self::build_tree(&options),
        }
    }

    pub fn number_of_branches(&self) -> usize {
        self.branches.len()
    }

    fn build_tree(options: &TreeOptions) -> Branches {
        let mut nodes = Branches::new();
        let start_node = Branch {
            origin: (0.0, 0.0),
            level: 0,
            heading: PI / 2.0,
            length: options.base_length,
        };
        Self::continue_tree(start_node, options, &mut nodes);

        nodes
    }

    fn continue_tree(current_node: Branch, options: &TreeOptions, nodes: &mut Branches) {
        nodes.push(current_node);

        let current_node = nodes.last().unwrap();

        if current_node.level == options.levels - 1 {
            return;
        }

        let next_pos = current_node.end();
        let next_headings = [
            current_node.heading - options.angle_diff,
            current_node.heading + options.angle_diff,
        ];
        let next_length = current_node.length * options.length_contraction;
        let next_level = current_node.level + 1;

        for heading in next_headings {
            Self::continue_tree(
                Branch {
                    level: next_level,
                    origin: next_pos,
                    heading,
                    length: next_length,
                },
                options,
                nodes,
            );
        }
    }
}

impl EguiDrawable for Tree {
    fn ui(&mut self, ui: &mut Ui) {
        self.options.ui(ui);
        let start = Instant::now();
        if self.options != self.last_options {
            // Rebuild the tree if the options have been changed
            self.branches = Self::build_tree(&self.options);
        }
        let duration = start.elapsed();
        self.last_options = self.options;

        ui.label(format!(
            "Rebuilding the tree took {} µs",
            duration.as_micros()
        ));

        let painter = ui.painter();
        let rect = painter.clip_rect();
        let to_screen =
            emath::RectTransform::from_to(Rect::from_center_size(Pos2::ZERO, vec2(2.0, 2.0)), rect);

        for node in &self.branches {
            let width =
                self.options.base_width * self.options.width_contraction.powi(node.level as i32);
            let stroke = Stroke::new(width, egui::Color32::DARK_RED);
            let end = node.end();
            painter.add(egui::Shape::LineSegment {
                points: [
                    to_screen * pos2(node.origin.0, remap(node.origin.1, 0.0..=2.0, 1.0..=-1.0)),
                    to_screen * pos2(end.0, remap(end.1, 0.0..=2.0, 1.0..=-1.0)),
                ],
                stroke,
            });
        }
    }
}
