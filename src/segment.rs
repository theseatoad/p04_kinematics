#[derive(Clone)]
pub struct Segment {
    a: (f32, f32),
    b: (f32, f32),
    len: f32,
    // must be stored as a radian
    angle: f32,
}

impl Segment {
    pub fn new(x: f32, y: f32, len: f32, angle: f32) -> Self {
        let mut seg = Segment {
            a: (x, y),
            b: (0.0, 0.0),
            len,
            angle,
        };
        seg.calculate_b();
        seg
    }

    pub fn calculate_b(&mut self) {
        let dx: f32 = self.len * self.angle.cos();
        let dy: f32 = self.len * self.angle.sin();
        self.b = (self.a.0 + dx, self.a.1 + dy);
    }
}

#[derive(Clone)]
pub struct Segments {
    segments: Vec<Segment>,
}

impl Segments {
    pub fn builder() -> SegmentsBuilder {
        SegmentsBuilder::default()
    }
}

#[derive(Default)]
pub struct SegmentsBuilder {
    segments: Vec<Segment>,
}

impl SegmentsBuilder {
    pub fn new(x: f32, y: f32, len: f32, angle: f32) -> SegmentsBuilder {
        let root = Segment::new(x, y, len, angle);
        SegmentsBuilder {
            segments: vec![root],
        }
    }

    // Adds the next segment ( Order matters )
    pub fn segment(mut self, len: f32, angle: f32) -> SegmentsBuilder {
        let prev_segment = self.segments.last().unwrap();
        let new_segment = Segment::new(prev_segment.b.0, prev_segment.b.1, len, angle);
        self.segments.push(new_segment);
        self
    }

    pub fn build(self) -> Segments {
        Segments {
            segments: self.segments,
        }
    }
}
