use super::schema::*;
use rocket_contrib::databases::diesel::{Insertable, Queryable};

#[derive(Queryable, Clone, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: Vec<u8>,
    pub privilege: i32,
}

#[derive(Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum Privilege {
    User = 0,
    Moderator = 1,
    Administrator = 2,
}

impl std::convert::TryFrom<i32> for Privilege {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == Self::User as i32 => Ok(Self::User),
            x if x == Self::Moderator as i32 => Ok(Self::Moderator),
            x if x == Self::Administrator as i32 => Ok(Self::Administrator),
            _ => Err(()),
        }
    }
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub username: &'a str,
    pub password: &'a [u8],
}

#[derive(Queryable, Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Model {
    pub id: i32,
    pub filename: String,
    pub material: Option<String>,
    pub texture: Option<String>,
    pub category: Option<String>,
}

#[derive(Insertable, Default)]
#[table_name = "models"]
pub struct NewModel<'a> {
    pub filename: &'a str,
    pub material: Option<&'a str>,
    pub texture: Option<&'a str>,
    pub category: Option<&'a str>,
}

#[derive(Queryable, Clone, Debug)]
pub struct LabelSet {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub model: i32,
}

#[derive(Insertable)]
#[table_name = "labelsets"]
pub struct NewLabelSet<'a> {
    pub id: Option<i32>,
    pub uuid: &'a str,
    pub name: &'a str,
    pub model: i32,
}

#[derive(Queryable, Clone)]
pub struct Label {
    pub id: i32,
    pub labelset: i32,
    pub name: String,
    pub colour: String,
    pub vertices: Vec<u8>,
}

#[derive(Insertable)]
#[table_name = "labels"]
pub struct NewLabel<'a> {
    pub labelset: i32,
    pub name: &'a str,
    pub colour: &'a str,
    pub vertices: &'a [u8],
}

#[derive(Queryable, Clone, Insertable)]
#[table_name = "userlabelsets"]
pub struct UserLabelSet {
    pub userid: i32,
    pub labelset: i32,
}

#[derive(Queryable, Clone, Insertable)]
#[table_name = "userquizzes"]
pub struct UserQuiz {
    pub userid: i32,
    pub quiz: i32,
}

#[derive(Queryable)]
pub struct Quiz {
    pub id: i32,
    pub uuid: String,
    pub name: String,
    pub labelset: i32,
    pub shuffle: i16,
}

#[derive(Insertable)]
#[table_name = "quizzes"]
pub struct NewQuiz<'a> {
    pub id: Option<i32>,
    pub uuid: &'a str,
    pub name: &'a str,
    pub labelset: i32,
    pub shuffle: i16,
}

#[derive(Queryable, Debug)]
pub struct Question {
    pub id: i32,
    pub quiz: i32,
    pub questiontype: i16,
    pub textprompt: String,
    pub textanswer: Option<String>,
    pub label: Option<i32>,
    pub showregions: i16,
}

#[derive(Insertable)]
#[table_name = "questions"]
pub struct NewQuestion<'a> {
    pub quiz: i32,
    pub questiontype: i16,
    pub textprompt: &'a str,
    pub textanswer: Option<&'a str>,
    pub label: Option<i32>,
    pub showregions: i16,
}
