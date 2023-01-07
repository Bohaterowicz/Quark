import React, {useState} from "react";

function PostList() {
    const [postsState, setPostsState] = useState("loading...");
    const [isUpdated, setIsUpdated] = useState(false);

    if(!isUpdated){
        fetch("http://127.0.0.1:1234/posts")
        .then(response => response.json())
        .then(data => { 
            setIsUpdated(true);
            let posts = [];
            posts.push(<div> {data.text} </div>);
            posts.push(<div> {data.text} </div>);
            posts.push(<div> {data.text} </div>);
            posts.push(<div> {data.text} </div>);
            posts.push(<div> {data.text} </div>);
            posts.push(<div> {data.text} </div>);
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