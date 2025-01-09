//#region EXAMPLE 1
struct Book {
    title: String,
    author: String,
    genre: String,
}

struct BookIterator {
    books: Vec<Book>,
}

impl Iterator for BookIterator {
    type Item = Book;

    fn next(&mut self) -> Option<Self::Item> {
        if self.books.len() > 0 {
            // remove function return value at the index and remove it from the vector
            Some(self.books.remove(0))
        } else {
            None
        }
    }
}

// this will allow us to have into_iter function on Book struct
// so we can easily convert a book instance into a iterable object.
impl IntoIterator for Book {
    type Item = Book;
    type IntoIter = BookIterator;

    fn into_iter(self) -> Self::IntoIter {
        BookIterator { books: vec![self] }
    }
}
//#endregion EXAMPLE 1

//#region EXAMPLE 2
struct User {
    fname: String,
    lname: String,
    username: String,
}

struct UserIterator {
    fields: Vec<String>,
}

impl Iterator for UserIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.fields.is_empty() {
            Some(self.fields.remove(0))
        } else {
            None
        }
    }
}

impl IntoIterator for User {
    type Item = String;
    type IntoIter = UserIterator;

    // this function return a vector of strings containing fname, lname and username of a given user as an array
    fn into_iter(self) -> Self::IntoIter {
        UserIterator {
            fields: vec![self.fname, self.lname, self.username],
        }
    }
}

//#endregion EXAMPLE 2

//#region EXAMPLE 3
// using vector into_iter
struct Student {
    name: String,
    identifier: String,
}

impl IntoIterator for Student {
    type Item = String;

    // using vector into iterator trait
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![self.name, self.identifier].into_iter()
    }
}
//#endregion EXAMPLE 3

fn main() {
    println!("============ EXAMPLE 1 =================");
    //#region example 1 usage
    let b_1 = Book {
        title: String::from("Some book"),
        author: String::from("MGholami"),
        genre: String::from("Computer"),
    };

    let book_as_iterator = b_1.into_iter();
    for b in book_as_iterator {
        println!("Book title is: {}", b.title);
    }
    //#endregion example 1 usage

    println!("============ EXAMPLE 2 =================");
    //#region example 2 usage
    let user = User {
        fname: "Kaveh".to_string(),
        lname: "Azadgan".to_string(),
        username: "_kaveh_".to_string(),
    };
    let user_properties = user.into_iter();
    for p in user_properties {
        println!("{p}");
    }
    //#endregion example 2 usage

    println!("============ EXAMPLE 3 =================");
    //#region example 3 usage
    let student = Student {
        name: "Mohammad Gholami".to_string(),
        identifier: "2025-01-02-19873".to_string(),
    };

    for f in student.into_iter() {
        println!("student prop: {}", f);
    }
    //#endregion example 3 usage
}
