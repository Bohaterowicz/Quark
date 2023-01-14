import React from "react";

function NewPostsNotifier(props) {
    let notifierDiv = <div></div>

    if(props.count > 0) {
        notifierDiv = 
        <div onClick={props.getNewPostsHandler} id="new_post_notifier">
            <p id="new_post_notifier_text">
                There are {props.count} new posts, show them
            </p>
        </div>
    } 
        
    return notifierDiv
}

export default NewPostsNotifier;