mod position;
mod cardinal;


use ::graphic;

pub use self::cardinal::Cardinal;
pub use self::position::Position;
use ::std;
use std::ops::BitAnd;

#[repr(C)]
#[derive(Copy)]
pub struct Persona {
    sheet: graphic::Sheet,
    emotion: [[graphic::Tuple; graphic::SPEC_MAX_XY]; graphic::SPEC_MAX_DRAW],
    position: Position,
}

impl Persona {
    pub fn get_sheet(&self) -> &graphic::Sheet {
        &self.sheet
    }

    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn get_emotion(&self) -> &[[graphic::Tuple; graphic::SPEC_MAX_XY]; graphic::SPEC_MAX_DRAW] {
        &self.emotion
    }

    pub fn set_sheet(&mut self, sheet: graphic::Sheet) {
        self.sheet = sheet;
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = position;
    }

    pub fn set_emotion(
        &mut self,
        emotion: [[graphic::Tuple; graphic::SPEC_MAX_XY]; graphic::SPEC_MAX_DRAW]
    ) {
        self.emotion = emotion;
    }
}

impl PartialEq for Persona {
    fn eq(&self, other: &Persona) -> bool {
        self.position.eq(&other.position).bitand(
            self.sheet.eq(&other.sheet).bitand(
                 self.emotion.iter()
                     .zip(other.emotion.iter())
                     .all(|(emotions, other_emotions):
                           (&[graphic::Tuple; graphic::SPEC_MAX_XY],
                            &[graphic::Tuple; graphic::SPEC_MAX_XY])| {
                         emotions.iter()
                                 .zip(other_emotions.iter())
                                 .all(|(emotion, other_emotion):
                                       (&graphic::Tuple,
                                        &graphic::Tuple)| {
                                     emotion.eq(other_emotion)
                                 })
                     })
             )
         )
    }
}

impl Clone for Persona {
    fn clone(&self) -> Self {
        unsafe {
            let mut emotion: [[graphic::Tuple; graphic::SPEC_MAX_XY]; graphic::SPEC_MAX_DRAW] = std::mem::uninitialized();
            emotion.copy_from_slice(&self.emotion);
            Persona {
                sheet: self.sheet,
                emotion: emotion,
                position: Position::default(),
            }
        }
    }
}

impl std::fmt::Display for Persona {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "Persona {{ sheet: {}, position: {:?} }}",
               self.sheet,
               self.position)
    }
}

impl std::fmt::Debug for Persona {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f,
               "Persona {{ sheet: {}, emotion: [{:?}, {:?}, {:?}, {:?}, ...], position: {:?} }}",
               self.sheet,
               &self.emotion[0][..8],
               &self.emotion[1][..8],
               &self.emotion[2][..8],
               &self.emotion[3][..8],
               self.position)
    }
}

impl Default for Persona {
    fn default() -> Self {
        let mut pos = Position::default();
        pos.set_cardinal(Cardinal::UpperRight);
        Persona {
            sheet: graphic::Sheet::default(),
            emotion: [[graphic::Tuple::default(); graphic::SPEC_MAX_XY]; graphic::SPEC_MAX_DRAW],
            position: pos,
        }
    }
}
