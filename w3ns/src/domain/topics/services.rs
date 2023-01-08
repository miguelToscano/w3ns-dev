use candid::Principal;
use ic_kit::ic;

use crate::domain::topics::types::Topic;
use crate::errors::ApiError;
use crate::repositories::topics::Topics;

pub fn get_owner_topics(owner: &Principal) -> Vec<Topic> {
    ic::with(|topics_repository: &Topics| topics_repository.get_topics(owner))
}

pub fn create_topic(owner: &Principal, topic: &Topic) -> Result<(), ApiError> {
    ic::with_mut(|topics_repository: &mut Topics| {
        let existing_topic = topics_repository.get_topic(owner, topic.clone().name);

        if existing_topic.is_some() {
            return Err(ApiError::TopicAlreadyExists);
        }

        topics_repository
            .add(owner, topic)
            .map_err(|_| ApiError::InternalError)
    })
}

pub fn delete_topic(owner: &Principal, topic_name: String) -> Result<(), ApiError> {
    ic::with_mut(|topics_repository: &mut Topics| {
        let existing_topic = topics_repository.get_topic(owner, topic_name.clone());

        if existing_topic.is_none() {
            return Err(ApiError::TopicNotFound);
        }

        topics_repository
            .delete(owner, topic_name)
            .map_err(|_| ApiError::InternalError)
    })
}