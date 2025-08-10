mod epic_detail_page;
mod home_page;
mod page;
mod story_detail_page;

pub use epic_detail_page::EpicDetail;
pub use home_page::HomePage;
pub use page::Page;
pub use story_detail_page::StoryDetail;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::db::mock_database::MockDB;
    use crate::models::{Epic, Story};

    mod home_page {
        use super::*;
        use crate::db::JiraDatabase;
        use crate::models::Action;
        use std::rc::Rc;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let page = HomePage { db };
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let page = HomePage { db };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic = Epic::new("".to_owned(), "".to_owned());

            let epic_id = db.create_epic(epic).unwrap();

            let page = HomePage { db };

            let q = "q";
            let c = "c";
            let valid_epic_id = epic_id.to_string();
            let invalid_epic_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "q983f2j";
            let input_with_trailing_white_spaces = "q\n";

            assert_eq!(page.handle_input(q).unwrap(), Some(Action::Exit));
            assert_eq!(page.handle_input(c).unwrap(), Some(Action::CreateEpic));
            assert_eq!(
                page.handle_input(&valid_epic_id).unwrap(),
                Some(Action::NavigateToEpicDetail { epic_id: 1 })
            );
            assert_eq!(page.handle_input(invalid_epic_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(
                page.handle_input(junk_input_with_valid_prefix).unwrap(),
                None
            );
            assert_eq!(
                page.handle_input(input_with_trailing_white_spaces).unwrap(),
                None
            );
        }
    }

    mod epic_detail_page {
        use super::*;
        use crate::db::JiraDatabase;
        use crate::models::Action;
        use std::rc::Rc;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });
            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();

            let page = EpicDetail { epic_id, db };
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });
            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();

            let page = EpicDetail { epic_id, db };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_epic_id() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let page = EpicDetail { epic_id: 999, db };
            assert_eq!(page.draw_page().is_err(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();
            let story_id = db
                .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
                .unwrap();

            let page = EpicDetail { epic_id, db };

            let p = "p";
            let u = "u";
            let d = "d";
            let c = "c";
            let invalid_story_id = "999";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(
                page.handle_input(p).unwrap(),
                Some(Action::NavigateToPreviousPage)
            );
            assert_eq!(
                page.handle_input(u).unwrap(),
                Some(Action::UpdateEpicStatus { epic_id: 1 })
            );
            assert_eq!(
                page.handle_input(d).unwrap(),
                Some(Action::DeleteEpic { epic_id: 1 })
            );
            assert_eq!(
                page.handle_input(c).unwrap(),
                Some(Action::CreateStory { epic_id: 1 })
            );
            assert_eq!(
                page.handle_input(&story_id.to_string()).unwrap(),
                Some(Action::NavigateToStoryDetail {
                    epic_id: 1,
                    story_id: 2
                })
            );
            assert_eq!(page.handle_input(invalid_story_id).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(
                page.handle_input(junk_input_with_valid_prefix).unwrap(),
                None
            );
            assert_eq!(
                page.handle_input(input_with_trailing_white_spaces).unwrap(),
                None
            );
        }
    }

    mod story_detail_page {
        use super::*;
        use crate::db::JiraDatabase;
        use crate::models::Action;
        use std::rc::Rc;

        #[test]
        fn draw_page_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();
            let story_id = db
                .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
                .unwrap();

            let page = StoryDetail {
                epic_id,
                story_id,
                db,
            };
            assert_eq!(page.draw_page().is_ok(), true);
        }

        #[test]
        fn handle_input_should_not_throw_error() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();
            let story_id = db
                .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
                .unwrap();

            let page = StoryDetail {
                epic_id,
                story_id,
                db,
            };
            assert_eq!(page.handle_input("").is_ok(), true);
        }

        #[test]
        fn draw_page_should_throw_error_for_invalid_story_id() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();
            let _ = db
                .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
                .unwrap();

            let page = StoryDetail {
                epic_id,
                story_id: 999,
                db,
            };
            assert_eq!(page.draw_page().is_err(), true);
        }

        #[test]
        fn handle_input_should_return_the_correct_actions() {
            let db = Rc::new(JiraDatabase {
                database: Box::new(MockDB::new()),
            });

            let epic_id = db
                .create_epic(Epic::new("".to_owned(), "".to_owned()))
                .unwrap();
            let story_id = db
                .create_story(Story::new("".to_owned(), "".to_owned()), epic_id)
                .unwrap();

            let page = StoryDetail {
                epic_id,
                story_id,
                db,
            };

            let p = "p";
            let u = "u";
            let d = "d";
            let some_number = "1";
            let junk_input = "j983f2j";
            let junk_input_with_valid_prefix = "p983f2j";
            let input_with_trailing_white_spaces = "p\n";

            assert_eq!(
                page.handle_input(p).unwrap(),
                Some(Action::NavigateToPreviousPage)
            );
            assert_eq!(
                page.handle_input(u).unwrap(),
                Some(Action::UpdateStoryStatus { story_id })
            );
            assert_eq!(
                page.handle_input(d).unwrap(),
                Some(Action::DeleteStory { epic_id, story_id })
            );
            assert_eq!(page.handle_input(some_number).unwrap(), None);
            assert_eq!(page.handle_input(junk_input).unwrap(), None);
            assert_eq!(
                page.handle_input(junk_input_with_valid_prefix).unwrap(),
                None
            );
            assert_eq!(
                page.handle_input(input_with_trailing_white_spaces).unwrap(),
                None
            );
        }
    }
}
