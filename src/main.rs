#[derive(Debug)]
struct User {
    id: u32,
    name: String,
}

struct Repositories<'a, U>
where
    U: UserRepo,
{
    user: &'a U,
}
impl<'a, U> Repositories<'a, U>
where
    U: UserRepo,
{
    fn new(user_repo: &'a U) -> Self {
        Self { user: user_repo }
    }
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
    let repo = Repositories::new(&user_repo_impl);
    let users = find_all(&repo);
    println!("{users}");
}

fn find_all<'a, U>(repo: &Repositories<'a, U>) -> String
where
    U: UserRepo,
{
    let users = repo.user.find_all().unwrap();
    format!("{}:{}", users[0].id, users[0].name)
}

#[test]
fn test_find_all() {
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
    let repo = Repositories::new(&mock_user_repo_impl);
    let users = find_all(&repo);
    assert_eq!(users, String::from("1:taro"));
}
