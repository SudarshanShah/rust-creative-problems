/*
    Build a feed system where any content type (Tweet, BlogPost, Meme) can appear in a unified feed, as long as
    it implements a Displayable trait with methods render(&self) -> String and likes(&self) -> u32. The feed is
    a Vec<Box<dyn Displayable>>. Write a generic function top_posts<T: Displayable> that takes a slice and
    returns the top N by likes. Ownership of posts belongs to the feed; viewers only borrow.
*/

/*
The Story

    You're building the backend content engine for "CringeBook" — a social media platform so chaotic it allows completely different content types in the same feed. A user's feed can have Tweets, BlogPosts, and Memes all jumbled together. Your job: build a unified feed system where any content type can participate, as long as  it follows the platform's contract.
*/

trait Displayable {
    fn render(&self) -> String;
    fn likes(&self) -> u32;
    fn content_type(&self) -> &str; // returns "Tweet", "BlogPost", "Meme"
}

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String, // max 280 chars (don't enforce, just know it)
    likes: u32,
    retweets: u32,
}

#[derive(Debug)]
struct BlogPost {
    title: String,
    author: String,
    body: String,
    likes: u32,
    read_time_mins: u8,
}

#[derive(Debug)]
struct Meme {
    title: String,
    likes: u32,
    spiciness: u8, // 1-10, how spicy the meme is 🌶️
}

impl Displayable for Tweet {
    fn render(&self) -> String {
        format!(
            "[Tweet] @{}\n\"{}\"\n👍 {} likes | 🔁 {} retweets \n",
            self.username, self.content, self.likes, self.retweets
        )
    }

    fn likes(&self) -> u32 {
        self.likes
    }

    fn content_type(&self) -> &str {
        "Tweet"
    }
}

impl Displayable for BlogPost {
    fn render(&self) -> String {
        format!(
            "[BlogPost] \"{}\" by {} \n 📖 {} min read \n {} \n 👍 {} likes \n",
            self.title, self.author, self.read_time_mins, self.body, self.likes
        )
    }

    fn likes(&self) -> u32 {
        self.likes
    }

    fn content_type(&self) -> &str {
        "BlogPost"
    }
}

impl Displayable for Meme {
    fn render(&self) -> String {
        format!(
            "[Meme] \"{}\" 🌶️ ({} / 10 spice)  \n 👍 {} likes \n",
            self.title, self.spiciness, self.likes
        )
    }

    fn likes(&self) -> u32 {
        self.likes
    }

    fn content_type(&self) -> &str {
        "Meme"
    }
}

// Task 1
struct Feed {
    posts: Vec<Box<dyn Displayable>>,
}

impl Feed {
    fn new() -> Self {
        Feed { posts: vec![] }
    }

    fn add(&mut self, post: Box<dyn Displayable>) {
        self.posts.push(post);
    }

    fn display_all(&self) {
        // prints every post's render()
        self.posts
            .iter()
            .for_each(|post| println!("{}", post.render()));
    }

    fn total_likes(&self) -> u32 {
        // sum of all post likes
        self.posts.iter().map(|post| post.likes()).sum()
    }

    fn filter_by_type<'a>(&'a self, content_type: &str) -> Vec<&'a dyn Displayable> {
        self.posts
            .iter()
            .filter(|post| content_type.eq(post.content_type()))
            .map(|post| post.as_ref())
            .collect()
    }
}

// A generic top_posts function (outside Feed):
fn top_posts<T: Displayable>(posts: &[T], n: usize) -> Vec<&T> {
    let mut vec: Vec<&T> = posts.iter().collect();

    vec.sort_by(|p1, p2| p2.likes().cmp(&p1.likes()));

    vec.iter().take(n).map(|post| *post).collect()
}

fn main() {
    println!("===== 📱 CringeBook Feed =====\n");

    let t1 = Tweet {
        username: "Sudarshan".to_string(),
        content: "claude is amazing".to_string(),
        likes: 1000,
        retweets: 510,
    };

    let t2 = Tweet {
        username: "Tony".to_string(),
        content: "jarvis is ultron".to_string(),
        likes: 10000,
        retweets: 4000,
    };

    let b1 = BlogPost {
        title: "Superman in the town!".to_string(),
        author: "Harry Potter".to_string(),
        body: "Superman, with his sister Supergirl in Hogwarts, its crazy!".to_string(),
        likes: 100_000,
        read_time_mins: 2,
    };

    let b2 = BlogPost {
        title: "Doomsday is coming!".to_string(),
        author: "Victor von Doom".to_string(),
        body: "Avengers! Your Dooomsday is arriving for real in December...Winter is coming!"
            .to_string(),
        likes: 1_000_000_000,
        read_time_mins: 10,
    };

    let m1 = Meme {
        title: "Claude got leaked, and open source!".to_string(),
        likes: 1200,
        spiciness: 10,
    };

    let m2 = Meme {
        title: "Lollygagging is real, Codex goes mute!".to_string(),
        likes: 1500,
        spiciness: 8,
    };

    let feed: Feed = Feed {
        posts: vec![
            Box::new(t1),
            Box::new(t2),
            Box::new(b1),
            Box::new(b2),
            Box::new(m1),
            Box::new(m2),
        ],
    };

    feed.display_all();

    println!("... (more posts)\n");

    println!("===== 📊 Feed Stats =====");
    println!("Total likes across all posts: {:?} \n", feed.total_likes());

    let t1 = Tweet {
        username: "Sudarshan".to_string(),
        content: "claude is amazing".to_string(),
        likes: 1000,
        retweets: 510,
    };

    let t2 = Tweet {
        username: "Tony".to_string(),
        content: "jarvis is ultron".to_string(),
        likes: 10000,
        retweets: 4000,
    };

    let t3 = Tweet {
        username: "Claude".to_string(),
        content: "Anthropic on fire!!!".to_string(),
        likes: 20000,
        retweets: 3000,
    };

    let t4 = Tweet {
        username: "Tony".to_string(),
        content: "jarvis is ultron".to_string(),
        likes: 5000,
        retweets: 10000,
    };

    let t5 = Tweet {
        username: "Tony".to_string(),
        content: "jarvis is ultron".to_string(),
        likes: 1000,
        retweets: 4000,
    };

    println!("===== 🏆 Top 2 Tweets =====");
    top_posts(&vec![t1, t2, t3, t4, t5], 2).iter().for_each(|p| println!("{}", p.render()));

    println!("===== 🎭 Memes Only =====");
    feed.filter_by_type("Meme").iter().for_each(|p| println!("{}", p.render()));
}
