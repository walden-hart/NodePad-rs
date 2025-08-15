use eframe::egui::{self, Color32, Pos2, Rect, Response, Sense, Stroke};
use crate::graph::Graph;

pub struct NodePadApp {
    graph: Graph,
}

impl NodePadApp {
    fn new(graph: Graph) -> Self {
        Self {
            graph
        }
    }

    fn draw_edges(&self, painter: &egui::Painter) {
        for edge in &self.graph.edges {
            if let (Some(from), Some(to)) = (
                self.graph.nodes.get(&edge.from),
                self.graph.nodes.get(&edge.to),
            ) {
                painter.line_segment(
                    [
                        Pos2::new(from.position.x, from.position.y),
                        Pos2::new(to.position.x, to.position.y),
                    ],
                    Stroke::new(2.0, Color32::BLACK),
                );
            }
        }
    }

    fn draw_nodes(&mut self, ui: &mut egui::Ui, painter: &egui::Painter) {
        for (id, node) in &mut self.graph.nodes {
            let node_rect = Rect::from_center_size(
                Pos2::new(node.position.x, node.position.y),
                egui::vec2(40.0, 40.0),
            );
            let response: Response =
                ui.interact(node_rect, ui.id().with(*id), Sense::click_and_drag());

            if response.dragged() {
                let delta = response.drag_delta();
                node.position.x += delta.x;
                node.position.y += delta.y;
            }

            painter.rect_filled(node_rect, 5.0, Color32::from_rgb(180, 200, 255));
            painter.rect_stroke(
                node_rect,
                5.0, // corner radius
                Stroke::new(2.0, Color32::BLACK),
                egui::StrokeKind::Middle
            );
            painter.text(
                Pos2::new(node.position.x, node.position.y),
                egui::Align2::CENTER_CENTER,
                &node.label,
                egui::TextStyle::Button.resolve(&ui.style()),
                Color32::BLACK,
            );
        }
    }
}


impl Default for NodePadApp {
    fn default() -> Self {
        let mut graph = Graph::new();
        let n1 = graph.add_node("A", Pos2::new(100.0, 100.0));
        let n2 = graph.add_node("B", Pos2::new(300.0, 150.0));
        let n3 = graph.add_node("C", Pos2::new(200.0, 300.0));
        graph.add_edge(n1, n2);
        graph.add_edge(n2, n3);
        graph.add_edge(n3, n1);

        Self::new(graph)
    }
}

impl eframe::App for NodePadApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter_at(ui.max_rect());

            // Draw edges
            self.draw_edges(&painter);

            self.draw_nodes(ui, &painter);
        });

        ctx.request_repaint();
    }
}