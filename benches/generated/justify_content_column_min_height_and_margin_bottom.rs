pub fn compute() -> stretch::layout::Node {
    stretch::compute(
        &stretch::style::Node {
            flex_direction: stretch::style::FlexDirection::Column,
            justify_content: stretch::style::JustifyContent::Center,
            min_size: stretch::geometry::Size {
                height: stretch::style::Dimension::Points(50f32),
                ..Default::default()
            },
            children: vec![stretch::style::Node {
                size: stretch::geometry::Size {
                    width: stretch::style::Dimension::Points(20f32),
                    height: stretch::style::Dimension::Points(20f32),
                    ..Default::default()
                },
                margin: stretch::geometry::Rect {
                    bottom: stretch::style::Dimension::Points(10f32),
                    ..Default::default()
                },
                ..Default::default()
            }],
            ..Default::default()
        },
        stretch::geometry::Size::undefined(),
    )
    .unwrap()
}