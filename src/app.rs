use crate::graph::Graph;
use eframe::egui::{self, Color32, ColorImage, Pos2, Rect, Response, Sense, Stroke};
use egui_async::{Bind, EguiAsyncPlugin};
use image::load_from_memory;
use log::{info, warn};
use rfd::AsyncFileDialog;

enum Screen {
    Start,
    Main,
}

pub struct NodePadApp {
    graph: Graph,
    selected_node: Option<usize>,
    show_node_editor: bool,
    show_note_editor: bool,
    screen: Screen,
    file_dialog: Bind<Vec<u8>, String>,
    picked_file: Option<Vec<u8>>,
    show_file_dialog: bool,
    background_image: Option<egui::TextureHandle>,
}

impl NodePadApp {
    fn new(graph: Graph) -> Self {
        Self {
            graph,
            selected_node: Option::None,
            show_node_editor: false,
            show_note_editor: false,
            screen: Screen::Start,
            file_dialog: Bind::default(),
            picked_file: None,
            show_file_dialog: false,
            background_image: None,
        }
    }

    fn draw_edges(&self, painter: &egui::Painter) {
        for edge in &self.graph.edges {
            if let (Some(from), Some(to)) = (
                self.graph.nodes.get(&edge.from),
                self.graph.nodes.get(&edge.to),
            ) {
                painter.line_segment(
                    [from.position, to.position],
                    Stroke::new(2.0, Color32::BLACK),
                );
            }
        }
    }

    fn draw_nodes(&mut self, ui: &mut egui::Ui, painter: &egui::Painter) {
        for (id, node) in &mut self.graph.nodes {
            let node_rect = Rect::from_center_size(node.position, egui::vec2(40.0, 40.0));
            let response: Response =
                ui.interact(node_rect, ui.id().with(*id), Sense::click_and_drag());

            if response.dragged() {
                let delta = response.drag_delta();
                node.position.x += delta.x;
                node.position.y += delta.y;
            }

            if response.clicked() {
                self.selected_node = Some(*id);
                self.show_node_editor = true;
            }

            painter.rect_filled(node_rect, 5.0, Color32::from_rgb(180, 200, 255));
            painter.rect_stroke(
                node_rect,
                5.0,
                Stroke::new(2.0, Color32::BLACK),
                egui::StrokeKind::Middle,
            );
            painter.text(
                node.position,
                egui::Align2::CENTER_CENTER,
                &node.label,
                egui::TextStyle::Button.resolve(ui.style()),
                Color32::BLACK,
            );
        }
    }

    fn show_toolbar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            if ui.button("Add Node").clicked() {
                self.graph.add_node("New", "", Pos2::new(150.0, 150.0));
            }
            if ui.button("Clear Graph").clicked() {
                self.graph.clear();
            }
        });
    }

    fn node_editor_window(&mut self, ctx: &egui::Context) {
        if let Some(id) = self.selected_node {
            let other_node_ids: Vec<usize> = self
                .graph
                .nodes
                .keys()
                .filter(|&&other_id| other_id != id)
                .copied()
                .collect();

            egui::Window::new("Edit Node")
                .open(&mut self.show_node_editor)
                .show(ctx, |ui| {
                    if let Some(node) = self.graph.nodes.get_mut(&id) {
                        ui.label("Label:");
                        ui.text_edit_singleline(&mut node.label);

                        ui.separator();
                        ui.label("Note:");
                        ui.text_edit_singleline(&mut node.note);

                        ui.separator();
                        if ui.button("Edit Note").clicked() {
                            self.show_note_editor = true;
                        }

                        ui.separator();
                        ui.label("Add Edge to:");

                        for other_id in &other_node_ids {
                            let label = &self.graph.nodes[other_id].label;
                            if ui.button(label).clicked() {
                                self.graph.add_edge(id, *other_id);
                            }
                        }
                    }
                });
        }
    }

    fn note_editor_window(&mut self, ctx: &egui::Context) {
        if let Some(id) = self.selected_node {
            let edge_labels: Vec<(usize, String)> = self
                .graph
                .edges
                .iter()
                .filter_map(|edge| {
                    if edge.from == id || edge.to == id {
                        let other_id = if edge.from == id { edge.to } else { edge.from };
                        self.graph
                            .nodes
                            .get(&other_id)
                            .map(|n| (other_id, n.label.clone()))
                    } else {
                        None
                    }
                })
                .collect();

            if let Some(node) = self.graph.nodes.get_mut(&id) {
                egui::Window::new("Edit Note")
                    .resizable(true)
                    .open(&mut self.show_note_editor)
                    .show(ctx, |ui| {
                        ui.label("Note:");
                        let text_edit_height = (ui.available_height() * 0.6).max(100.0); // 60% of window height, at least 100 px
                        egui::ScrollArea::vertical()
                            .max_height(text_edit_height)
                            .show(ui, |ui| {
                                ui.add_sized(
                                    [ui.available_width(), ui.available_height()],
                                    egui::TextEdit::multiline(&mut node.note),
                                );
                            });

                        ui.separator();
                        ui.label("Links:");

                        for (target_id, target_label) in edge_labels {
                            if ui.link(target_label).clicked() {
                                self.selected_node = Some(target_id);
                            }
                        }
                    });
            }
        }
    }

    fn load_background(&mut self, ctx: &egui::Context) {
        let file = match self.picked_file.take() {
            Some(b) => b,
            None => return,
        };

        if let Ok(img) = load_from_memory(&file) {
            let size = [img.width() as usize, img.height() as usize];
            let rgb = img.to_rgba8();
            let pixels = rgb.as_flat_samples();

            let color_image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());

            self.background_image =
                Some(ctx.load_texture("background", color_image, Default::default()));
        }
    }

    fn draw_background(&self, ui: &mut egui::Ui) {
        if let Some(texture) = &self.background_image {
            let rect = ui.max_rect();
            ui.painter().image(
                texture.id(),
                rect,
                egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }
    }

    fn load_file(&mut self, filter_name: &'static str, filter_types: &'static [&'static str]) {
        if let Some(picked_file) = self.file_dialog.read_or_request(|| async move {
            let file = AsyncFileDialog::new()
                .add_filter(filter_name, filter_types)
                .set_directory("/")
                .pick_file()
                .await
                .ok_or("Problem Picking File".to_string())?;

            let bytes = file.read().await;
            Ok::<Vec<u8>, String>(bytes)
        }) {
            match picked_file {
                Ok(file) => {
                    info!("Loaded file");
                    self.picked_file = Some(file.clone());
                    self.show_file_dialog = false;
                }
                Err(e) => {
                    warn!("{e}");
                    self.show_file_dialog = false;
                }
            }

            self.file_dialog.clear();
        }
    }

    fn start_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("Start").clicked() {
                self.screen = Screen::Main
            }
            if ui.button("Pick File").clicked() {
                self.show_file_dialog = true;
            }
            if self.show_file_dialog {
                self.load_file("Image", &["png", "jpg"]);
            }

            self.load_background(ctx);
        });
    }

    fn main_screen(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter_at(ui.max_rect());

            self.draw_background(ui);

            self.show_toolbar(ui);

            self.draw_edges(&painter);

            self.draw_nodes(ui, &painter);
        });

        self.node_editor_window(ctx);
        self.note_editor_window(ctx);
    }
}

impl Default for NodePadApp {
    fn default() -> Self {
        let mut graph = Graph::new();
        let n1 = graph.add_node("A", "", Pos2::new(100.0, 100.0));
        let n2 = graph.add_node("B", "", Pos2::new(300.0, 150.0));
        let n3 = graph.add_node("C", "", Pos2::new(200.0, 300.0));
        graph.add_edge(n1, n2);
        graph.add_edge(n2, n3);
        graph.add_edge(n3, n1);

        Self::new(graph)
    }
}

impl eframe::App for NodePadApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        ctx.plugin_or_default::<EguiAsyncPlugin>();

        match self.screen {
            Screen::Main => self.main_screen(ctx),
            Screen::Start => self.start_screen(ctx),
        }

        ctx.request_repaint();
    }
}
