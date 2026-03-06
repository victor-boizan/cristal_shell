
use crate::messages::Message;
use iced::{ Element, Theme, Task };
use super::SurfaceType;

use iced_layershell::{
    reexport::{Anchor,Layer,KeyboardInteractivity},
};
 use iced_layershell::reexport::NewLayerShellSettings;

use wayland_client::{
    protocol::{wl_output},
};
use crate::messages::{Update};

use super::Surface;
#[derive(Default,Clone)]
pub struct Corners;
use iced::{
    Length, Rectangle,
    widget::{canvas, Canvas},
    mouse, Renderer,
};

#[derive(Default)]
pub struct CornersState;
impl<Message> canvas::Program<Message> for CornersState {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());
        
        let radius = 20.0;
        let nw = canvas::Path::new(|builder| {
            let origin = iced::Point{x:0.0,y:0.0};
            let center = iced::Point{x:radius,y:radius};
            builder.move_to(origin);
            builder.line_to(iced::Point{x:origin.x,y:center.y});
            builder.quadratic_curve_to(origin,iced::Point{x:center.x,y:origin.y});
            builder.close();
        });
        let ne = canvas::Path::new(|builder| {
            let origin = iced::Point{y:0.0,x:bounds.width};
            let center = iced::Point{y:radius,x:bounds.width-radius};
            builder.move_to(origin);
            builder.line_to(iced::Point{x:origin.x,y:center.y});
            builder.quadratic_curve_to(origin,iced::Point{x:center.x,y:origin.y});
            builder.close();
        });
        let sw = canvas::Path::new(|builder| {
            let origin = iced::Point{y:bounds.height,x:0.0};
            let center = iced::Point{y:bounds.height-radius,x:radius};
            builder.move_to(origin);
            builder.line_to(iced::Point{x:origin.x,y:center.y});
            builder.quadratic_curve_to(origin,iced::Point{x:center.x,y:origin.y});
            builder.close();
        });
        let se = canvas::Path::new(|builder| {
            let origin = iced::Point{y:bounds.height,x:bounds.width};
            let center = iced::Point{y:bounds.height-radius,x:bounds.width-radius};
            builder.move_to(origin);
            builder.line_to(iced::Point{x:origin.x,y:center.y});
            builder.quadratic_curve_to(origin,iced::Point{x:center.x,y:origin.y});
            builder.close();
        });

        let bar_side_fill = canvas::Fill{style: iced::widget::canvas::Style::Solid(theme.clone().palette().background),
            rule: iced::widget::canvas::fill::Rule::NonZero};
        let empty_side_fill = canvas::Fill{style: iced::widget::canvas::Style::Solid(iced::Color::BLACK),
            rule: iced::widget::canvas::fill::Rule::NonZero};
        frame.fill(&nw,bar_side_fill);
        frame.fill(&ne,empty_side_fill);
        frame.fill(&se,empty_side_fill);
        frame.fill(&sw,bar_side_fill);

        vec![frame.into_geometry()]
    }
}


impl Surface for Corners {
    fn layer_settings(&self,output: wl_output::WlOutput) -> NewLayerShellSettings {
        NewLayerShellSettings {
            size: Some((0, 0)),
            //exclusive_zone: None,
            exclusive_zone: Some(-1),
            anchor: Anchor::all(),
            layer: Layer::Top,
            //margin: None,
            margin:Some((0,0,0,30)),//Up,Right,Down,Left
            keyboard_interactivity: KeyboardInteractivity::None,
            events_transparent: true,
            namespace: Some(String::from("Cristal_Corners")),
            output_option: iced_layershell::reexport::OutputOption::Output(output.clone()),
        }

    }
    fn get_type(&self) -> SurfaceType { SurfaceType::Corners }
    fn update(&mut self, _update: Update) -> Task<Message> { Task::none() }
    fn view(&self) -> Element<'_,Message>{
            Canvas::new(CornersState::default())
                .width(Length::Fill)
                .height(Length::Fill)
        .into()
    }
}
impl Corners {
    pub fn new() -> Self{Self{}}
}
