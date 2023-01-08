import React, {useState} from "react";

function CreatePost(name, text) {
    var post =
        <div className="post">
            <div className="post_header">
                <a>
                    <img className="user_avatar" src="/ae.png" alt="User Avatar"/>
                </a>
                <div className="post_header_inner">
                    <div>
                        <p className="user_name">{name}</p>
                    </div>
                </div>
            </div>
            <div className="post_text">
                <p>{text}</p>
            </div>
        </div>
    return post
}

function PostList() {
    const [postsState, setPostsState] = useState("loading...");
    const [isUpdated, setIsUpdated] = useState(false);

    if(!isUpdated){
        fetch("http://127.0.0.1:1234/posts")
        .then(response => response.json())
        .then(data => { 
            setIsUpdated(true);
            let posts = [];
            const userPosts = data.posts
            for(let i = 0; i < userPosts.length; i++) {
                const post = CreatePost(userPosts[i].name, userPosts[i].text)
                posts.push(post);
            }
            setPostsState(posts);
        });
    }

    return (
        <div id="postList">
            {postsState}
        </div>
    )
}

export default PostList;