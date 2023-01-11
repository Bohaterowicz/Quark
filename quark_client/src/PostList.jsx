import React from "react";

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

function PostList(props) {
    const posts = props.posts
    if(posts.length == 0) {
        return <p>Loading...</p>
    }
    else {
        const postElements = []
        for(let i = 0; i < posts.length; i++){
            const postElement = CreatePost(posts[i].username, posts[i].post_content)
            postElements.push(postElement)
        }
        return (
            <div id="postList"> {postElements} </div>
        )
    }
}

export default PostList;