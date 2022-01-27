#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

struct RepoImpls<'a> {
    user: &'a UserRepoImpl
}
impl<'a> RepoImpls<'a> {
    fn new(user_repo_impl: &'a UserRepoImpl) -> Self {
        Self { user: user_repo_impl }
    }
}
trait Repositories<'a> {
    type UserRepoImpl: UserRepo;
    fn user(&self) -> &'a Self::UserRepoImpl;
}
impl<'a> Repositories<'a> for RepoImpls<'a> {
    type UserRepoImpl = UserRepoImpl;
    fn user(&self) -> &'a Self::UserRepoImpl { &self.user }
}

struct UserRepoImpl {}
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

fn find_all<'a, R:Repositories<'a>>(repo: &'a R) -> String {
    let users = repo.user().find_all().unwrap();
    format!("{}:{}", users[0].id, users[0].name)
}

#[test]
fn test_find_all() {
    struct MockRepoImpls<'a> {
        user: &'a MockUserRepoImpl
    }
    impl<'a> MockRepoImpls<'a> {
        fn new(user_repo_impl: &'a MockUserRepoImpl) -> Self {
            Self { user: user_repo_impl }
        }
    }
    impl<'a> Repositories<'a> for MockRepoImpls<'a> {
        type UserRepoImpl = MockUserRepoImpl;
        fn user(&self) -> &'a Self::UserRepoImpl { &self.user }
    }
    struct MockUserRepoImpl {}
    impl UserRepo for MockUserRepoImpl {
        fn find_all(&self) -> Result<Vec<User>, String> {
            let user_fixture = User {
                id: 1,
                name: String::from("taro"),
            };
            Ok(vec![user_fixture])
        }
    }
    let mock_user_repo_impl = MockUserRepoImpl {};
    let repo_impls = MockRepoImpls::new(&mock_user_repo_impl);
    let users = find_all(&repo_impls);
    assert_eq!(users, String::from("1:taro"));
}
