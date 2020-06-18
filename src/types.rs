/// Reddit base class.
/// Listing things have neither name nor id because they are indefinite objects.
/// That is, they are system generated, not user submitted, and are subject to change quickly and expire.
/// See https://github.com/reddit-archive/reddit/wiki/JSON#thing-reddit-base-class
#[derive(Debug, serde::Deserialize)]
pub struct Thing {
    /// this item's identifier, e.g. "8xwlg"
    pub id: Option<String>,

    /// Fullname of comment, e.g. "t1_c3v7f8u"
    pub name: Option<String>,

    /// Data
    #[serde(flatten)]
    pub data: ThingData,
}

/// kind:
/// All things have a kind. The kind is a String identifier that denotes the object's type.
/// Some examples: Listing, more, t1, t2
/// data:
/// A custom data structure used to hold valuable information.
/// This object's format will follow the data structure respective of its kind. See below for specific structures.
/// See https://www.reddit.com/dev/api#fullnames
#[derive(Debug, serde::Deserialize)]
#[serde(tag = "kind", content = "data")]
pub enum ThingData {
    #[serde(rename = "Listing")]
    Listing(Box<Listing>),

    // More is small + it already has a vector of things as a vec
    #[serde(rename = "more")]
    More(More),

    #[serde(rename = "t1")]
    Comment(Box<Comment>),

    // TODO: Finish type
    // #[serde(rename = "t2")]
    // Account(serde_json::Value),
    #[serde(rename = "t3")]
    Link(Box<Link>),
}

impl ThingData {
    /// Tries to get this ThingData as a listing
    pub fn as_listing(&self) -> Option<&Listing> {
        match self {
            ThingData::Listing(listing) => Some(listing),
            _ => None,
        }
    }

    /// Tries to get this ThingData as a mutable listing
    pub fn as_listing_mut(&mut self) -> Option<&mut Listing> {
        match self {
            ThingData::Listing(listing) => Some(listing),
            _ => None,
        }
    }

    /// Tries to turn this ThingData into a listing
    pub fn into_listing(self) -> Option<Box<Listing>> {
        match self {
            ThingData::Listing(listing) => Some(listing),
            _ => None,
        }
    }

    /// Tries to get this ThingData as a link
    pub fn as_link(&self) -> Option<&Link> {
        match self {
            ThingData::Link(link) => Some(link),
            _ => None,
        }
    }

    /// Tries to turn this ThingData into a link
    pub fn into_link(self) -> Option<Box<Link>> {
        match self {
            ThingData::Link(link) => Some(link),
            _ => None,
        }
    }
}

/// Used to paginate content that is too long to display in one go.
/// Add the query argument before or after with the value given to get the previous or next page.
/// This is usually used in conjunction with a count argument.
/// Exception: Unlike the other classes documented on this page, Listing is not a thing subclass, as it inherits directly from the Python base class, Object.
/// Although it does have data and kind as parameters, it does not have id and name.
/// A listing's kind will always be Listing and its data will be a List of things.
/// kind == "Listing"
/// See https://github.com/reddit-archive/reddit/wiki/JSON#listing
#[derive(Debug, serde::Deserialize)]
pub struct Listing {
    /// The fullname of the listing that follows before this page. null if there is no previous page.
    pub before: Option<String>,

    /// The fullname of the listing that follows after this page. null if there is no next page.
    pub after: Option<String>,

    /// This modhash is not the same modhash provided upon login.
    /// You do not need to update your user's modhash everytime you get a new modhash.
    /// You can reuse the modhash given upon login.
    pub modhash: String,

    /// A list of things that this Listing wraps.
    pub children: Vec<Thing>,
}

/// Implementation
/// See: https://github.com/reddit-archive/reddit/wiki/JSON#votable-implementation
#[derive(Debug, serde::Deserialize)]
pub struct Votable {
    /// the number of upvotes. (includes own)
    pub ups: i64,

    /// the number of downvotes. (includes own)
    pub downs: u64,

    /// true if thing is liked by the user, false if thing is disliked, null if the user has not voted or you are not logged in.
    /// Certain languages such as Java may need to use a boolean wrapper that supports null assignment.
    pub likes: Option<bool>,
}

/// Implementation
/// See https://github.com/reddit-archive/reddit/wiki/JSON#created-implementation
#[derive(Debug, serde::Deserialize)]
pub struct Created {
    // TODO: use chrono?
    /// the time of creation in local epoch-second format. ex: 1331042771.0
    pub created: f64,

    /// the time of creation in UTC epoch-second format. Note that neither of these ever have a non-zero fraction.
    pub created_utc: f64,
}

/// Implements votable | created
/// kind == "t1"
/// See https://github.com/reddit-archive/reddit/wiki/JSON#comment-implements-votable--created
#[derive(Debug, serde::Deserialize)]
pub struct Comment {
    /// who approved this comment. null if nobody or you are not a mod
    pub approved_by: Option<String>,

    /// the account name of the poster
    pub author: String,

    /// the CSS class of the author's flair. subreddit specific
    pub author_flair_css_class: Option<String>,

    /// the text of the author's flair. subreddit specific
    pub author_flair_text: Option<String>,

    /// who removed this comment. null if nobody or you are not a mod
    pub banned_by: Option<String>,

    /// the raw text.
    /// this is the unformatted text which includes the raw markup characters such as ** for bold. <, >, and & are escaped.
    pub body: String,

    /// the formatted HTML text as displayed on reddit.
    /// For example, text that is emphasised by * will now have <em> tags wrapping it.
    /// Additionally, bullets and numbered lists will now be in HTML list format.
    /// NOTE: The HTML string will be escaped. You must unescape to get the raw HTML.
    pub body_html: String,

    /// false if not edited, edit date in UTC epoch-seconds otherwise.
    /// NOTE: for some old edited comments on reddit.com, this will be set to true instead of edit date.
    pub special: Option<serde_json::Value>,

    /// the number of times this comment received reddit gold
    pub gilded: u64,

    /// how the logged-in user has voted on the comment - True = upvoted, False = downvoted, null = no vote
    pub likes: Option<bool>,

    /// present if the comment is being displayed outside its thread (user pages, /r/subreddit/comments/.json, etc.).
    /// Contains the author of the parent link
    pub link_author: Option<String>,

    /// ID of the link this comment is in
    pub link_id: String,

    /// present if the comment is being displayed outside its thread (user pages, /r/subreddit/comments/.json, etc.).
    /// Contains the title of the parent link
    pub link_title: Option<String>,

    /// present if the comment is being displayed outside its thread (user pages, /r/subreddit/comments/.json, etc.).
    /// Contains the URL of the parent link
    pub link_url: Option<String>,

    /// how many times this comment has been reported, null if not a mod
    pub num_reports: Option<u64>,

    /// ID of the thing this comment is a reply to, either the link or a comment in it
    pub parent_id: String,

    // TODO: Find out why this is a string sometimes
    /// A list of replies to this comment
    // pub replies: Thing,

    /// true if this post is saved by the logged in user
    pub saved: bool,

    /// the net-score of the comment
    pub score: i64,

    /// Whether the comment's score is currently hidden.
    pub score_hidden: bool,

    /// subreddit of thing excluding the /r/ prefix. "pics"
    pub subreddit: String,

    /// the id of the subreddit in which the thing is locatedss
    pub subreddit_id: String,

    /// to allow determining whether they have been distinguished by moderators/admins.
    /// null = not distinguished.
    /// moderator = the green \[M\].
    /// admin = the red \[A\].
    /// special = various other special distinguishes http://redd.it/19ak1b
    pub distinguished: Option<String>,

    /// Voting Implementation
    #[serde(flatten)]
    pub votable: Votable,

    /// Created Implementation
    #[serde(flatten)]
    pub created: Created,
    // Experimentally determined fields
    // TODO: These are VERY best-effort, but i should still try to document what i can
}

/// Implements votable | created
/// kind == "t3"
/// See https://github.com/reddit-archive/reddit/wiki/JSON#link-implements-votable--created
#[derive(Debug, serde::Deserialize)]
pub struct Link {
    /// the account name of the poster. null if this is a promotional link
    pub author: String,

    /// the CSS class of the author's flair. subreddit specific
    pub author_flair_css_class: Option<String>,

    /// the text of the author's flair. subreddit specific
    pub author_flair_text: Option<String>,

    /// probably always returns false
    pub clicked: bool,

    /// the domain of this link.
    /// Self posts will be self.<subreddit> while other examples include en.wikipedia.org and s3.amazon.com
    pub domain: String,

    /// true if the post is hidden by the logged in user. false if not logged in or not hidden.
    pub hidden: bool,

    /// true if this link is a selfpost
    pub is_self: bool,

    /// how the logged-in user has voted on the link - True = upvoted, False = downvoted, null = no vote
    pub likes: Option<bool>,

    /// the CSS class of the link's flair.
    pub link_flair_css_class: Option<String>,

    /// the text of the link's flair.
    pub link_flair_text: Option<String>,

    /// whether the link is locked (closed to new comments) or not.
    pub locked: bool,

    // TODO: Finish type
    /// Used for streaming video. Detailed information about the video and it's origins are placed here
    pub media: serde_json::Value,

    // TODO: Finish type
    /// Used for streaming video. Technical embed specific information is found here.
    pub media_embed: serde_json::Value,

    /// the number of comments that belong to this link. includes removed comments.
    pub num_comments: u64,

    /// true if the post is tagged as NSFW. False if otherwise
    pub over_18: bool,

    /// relative URL of the permanent link for this link
    pub permalink: String,

    /// true if this post is saved by the logged in user
    pub saved: bool,

    /// the net-score of the link.
    /// note: A submission's score is simply the number of upvotes minus the number of downvotes.
    /// If five users like the submission and three users don't it will have a score of 2.
    /// Please note that the vote numbers are not "real" numbers, they have been "fuzzed" to prevent spam bots etc.
    /// So taking the above example, if five users upvoted the submission, and three users downvote it,
    /// the upvote/downvote numbers may say 23 upvotes and 21 downvotes, or 12 upvotes, and 10 downvotes.
    /// The points score is correct, but the vote totals are "fuzzed".
    pub score: i64,

    /// the raw text.
    /// this is the unformatted text which includes the raw markup characters such as ** for bold.
    /// <, >, and & are escaped.
    /// Empty if not present.
    pub selftext: String,

    /// the formatted escaped HTML text.
    /// this is the HTML formatted version of the marked up text.
    /// Items that are boldened by ** or *** will now have <em> or *** tags on them.
    /// Additionally, bullets and numbered lists will now be in HTML list format.
    /// NOTE: The HTML string will be escaped. You must unescape to get the raw HTML. Null if not present.
    pub selftext_html: Option<String>,

    /// subreddit of thing excluding the /r/ prefix. "pics"
    pub subreddit: String,

    /// the id of the subreddit in which the thing is located
    pub subreddit_id: String,

    /// full URL to the thumbnail for this link;
    /// "self" if this is a self post;
    /// "image" if this is a link to an image but has no thumbnail;
    /// "default" if a thumbnail is not available
    pub thumbnail: String,

    /// the title of the link. may contain newlines for some reason
    pub title: String,

    /// the link of this post. the permalink if this is a self-post
    pub url: String,

    /// Indicates if link has been edited.
    /// Will be the edit timestamp if the link has been edited and return false otherwise.
    /// https://github.com/reddit/reddit/issues/581
    pub edited: serde_json::Value,

    /// to allow determining whether they have been distinguished by moderators/admins.
    /// null = not distinguished.
    /// moderator = the green \[M\].
    /// admin = the red \[A\].
    /// special = various other special distinguishes
    /// http://bit.ly/ZYI47B
    pub distinguished: Option<String>,

    /// true if the post is set as the sticky in its subreddit.
    pub stickied: bool,

    /// Voting Implementation
    #[serde(flatten)]
    pub votable: Votable,

    /// Created Implementation
    #[serde(flatten)]
    pub created: Created,

    // Experimentally determined fields
    // TODO: These are VERY best-effort, but i should still try to document what i can
    pub archived: bool,
    pub author_flair_template_id: Option<String>,
    pub author_flair_text_color: Option<String>,
    pub author_flair_type: Option<String>,
    pub author_fullname: Option<String>,
    pub author_patreon_flair: Option<bool>,
    pub can_gild: bool,
    pub can_mod_post: bool,
    pub contest_mode: bool,

    /// I believe that this is included for crossposted posts to get the data for the main post
    pub crosspost_parent_list: Option<Vec<Link>>,

    pub gilded: u64,
    pub hide_score: bool,
    pub id: String,
    pub is_crosspostable: bool,
    pub is_meta: bool,
    pub is_original_content: bool,
    pub is_reddit_media_domain: bool,
    pub is_robot_indexable: bool,

    /// Returns true if its a video
    pub is_video: bool,

    pub link_flair_text_color: String,
    pub link_flair_type: String,
    pub media_only: bool,
    pub name: String,
    pub no_follow: bool,
    pub num_crossposts: u64,
    pub parent_whitelist_status: Option<String>,

    /// Whether this post is pinned
    pub pinned: bool,

    /// A "hint" about what this post may be
    pub post_hint: Option<PostHint>,

    pub pwls: Option<u64>,
    pub quarantine: bool,
    pub send_replies: bool,

    /// Whether this post has a spoiler
    pub spoiler: bool,

    pub subreddit_name_prefixed: String,
    pub subreddit_subscribers: u64,
    pub subreddit_type: String,
    pub suggested_sort: Option<String>,
    pub thumbnail_height: Option<u32>,
    pub thumbnail_width: Option<u32>,
    pub visited: bool,
    pub whitelist_status: Option<String>,
    pub wls: Option<u32>,
}

/// kind == "more"
/// See https://github.com/reddit-archive/reddit/wiki/JSON#more
#[derive(Debug, serde::Deserialize)]
pub struct More {
    /// A list of String ids that are the additional things that can be downloaded but are not because there are too many to list.
    pub children: Vec<String>,
}

/// Info on what the post may contain
#[derive(Debug, serde::Deserialize, PartialEq)]
pub enum PostHint {
    /// The post is an image
    #[serde(rename = "image")]
    Image,

    /// The post is a link
    #[serde(rename = "link")]
    Link,

    /// A video hosted on reddit
    #[serde(rename = "hosted:video")]
    HostedVideo,

    #[serde(rename = "rich:video")]
    RichVideo,

    #[serde(rename = "self")]
    DataSelf,
}

#[cfg(test)]
mod test {
    use super::*;

    const SUBREDDIT_SAMPLE_1: &str = include_str!("../test_data/subreddit_dankmemes.json");
    const COMMENT_SAMPLE_1: &str = include_str!("../test_data/comment_h966lq.json");
    const COMMENT_SAMPLE_2: &str = include_str!("../test_data/comment_h8p0py.json");

    #[test]
    fn parse_subreddit() {
        let res = serde_json::from_str::<Thing>(SUBREDDIT_SAMPLE_1).unwrap();
        dbg!(res);
    }

    #[test]
    fn parse_comments_1() {
        let res = serde_json::from_str::<Vec<Thing>>(COMMENT_SAMPLE_1).unwrap();
        dbg!(res);
    }

    #[test]
    fn parse_comments_2() {
        let res = serde_json::from_str::<Vec<Thing>>(COMMENT_SAMPLE_2).unwrap();
        dbg!(res);
    }
}
