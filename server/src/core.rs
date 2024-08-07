#![allow(dead_code)]

mod stub {
    pub struct Set<T>(T);
    pub struct IndexSet<T>(T);
    pub struct DateTime;
}

use stub::*;

struct Hub {
    id: HubId,
    name: String,
    items: Set<HubItem>,
}

type HubId = usize;

enum HubItem {
    Hub(HubId),
    Blog(BlogId),
}

struct Blog {
    id: BlogId,
    name: String,
    description: Wiki,
    tags: Set<Tag>,
    hubs: Set<HubId>,
    posts: IndexSet<PostId>,
}

type BlogId = usize;

struct Post {
    id: PostId,
    body: Wiki<PostBody>,
    comments: IndexSet<CommentId>,
}

type PostId = usize;

struct PostBody {
    header: Wiki,
    content: Wiki,
    blogs: Wiki<Set<BlogId>>,
    tags: Wiki<Set<Tag>>,
}

enum Tag {
    Attr(Attribute),
    Text(String),
}

struct Attribute {
    name: String,
    value: AttributeValue,
}

enum AttributeValue {
    Int(u64),
    Real(f64),
    DateTime(DateTime),
    Text(String),
}

struct Comment {
    id: CommentId,
    parent_id: CommentParentId,
    content: Wiki,
    answers: IndexSet<CommentId>,
}

type CommentId = usize;

enum CommentParentId {
    Post(RevisedId<PostId>),
    Comment(RevisedId<CommentId>),
}

struct RevisedId<ID> {
    item: ID,
    revision: RevisionId,
}

struct Wiki<T = String> {
    last_content: T,
    revisions: IndexSet<RevisionId>,
}

struct Revision<U = TextUpdate> {
    id: RevisionId,
    author: Author,
    moment: DateTime,
    updates: IndexSet<U>,
}

type RevisionId = usize;

enum TextUpdate {
    Add {
        start_line: usize,
        text: String,
    },
    Del {
        start_line: usize,
        line_count: usize,
    },
    Change {
        start_line: usize,
        line_count: usize,
        text: String,
    },
}

enum Author {
    One(One),
    Group(Set<One>),
}

enum One {
    Person(Person),
    Community(Box<Community>),
}

struct Community {
    name: String,
    members: Set<One>,
}

struct Person {
    name: String,
}
