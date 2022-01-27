use mockall::automock;

#[derive(Debug, Clone)]
pub struct User {
    pub id: u32,
    pub name: String,
}

struct RepoImpls<'a> {
    user: &'a UserRepoImpl,
}
impl<'a> RepoImpls<'a> {
    fn new(user_repo_impl: &'a UserRepoImpl) -> Self {
        Self {
            user: user_repo_impl,
        }
    }
}
trait Repositories<'a> {
    type UserRepoImpl: UserRepo;
    fn user(&self) -> &'a Self::UserRepoImpl;
}
impl<'a> Repositories<'a> for RepoImpls<'a> {
    type UserRepoImpl = UserRepoImpl;
    fn user(&self) -> &'a Self::UserRepoImpl {
        &self.user
    }
}

pub struct UserRepoImpl {}
#[automock]
trait UserRepo {
    fn find_all(&self) -> Result<Vec<User>, String>;
}
impl UserRepo for UserRepoImpl {
    fn find_all(&self) -> Result<Vec<User>, String> {
        let user = User {
            id: 10,
            name: String::from("Bob"),
        };
        Ok(vec![user])
    }
}

fn main() {
    let user_repo_impl = UserRepoImpl {};
    let repo_impls = RepoImpls::new(&user_repo_impl);
    let users = find_all(&repo_impls);
    println!("{users}");
}

fn find_all<'a, R: Repositories<'a>>(repo: &'a R) -> String {
    let users = repo.user().find_all().unwrap();
    format!("{}:{}", users[0].id, users[0].name)
}

#[test]
fn test_find_all() {
    struct MockRepoImpls<'a> {
        user: &'a MockUserRepo,
    }
    impl<'a> MockRepoImpls<'a> {
        fn new(mock_user_repo_impl: &'a MockUserRepo) -> Self {
            Self {
                user: mock_user_repo_impl,
            }
        }
    }
    impl<'a> Repositories<'a> for MockRepoImpls<'a> {
        type UserRepoImpl = MockUserRepo;
        fn user(&self) -> &'a Self::UserRepoImpl {
            &self.user
        }
    }
    let mut mock_user_repo_impl = MockUserRepo::new();
    let user_fixture = User {
        id: 1,
        name: String::from("taro"),
    };
    mock_user_repo_impl
        .expect_find_all()
        .returning( move || Ok(vec![user_fixture.clone()]));
    let mock_repo_impls = MockRepoImpls::new(&mock_user_repo_impl);
    let users = find_all(&mock_repo_impls);
    assert_eq!(users, String::from("1:taro"));
}
