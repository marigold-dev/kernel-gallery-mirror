import { Tweet } from '../lib/tweet';
import TweetComponent from './TweetComponent';
import './Feed.css';

interface FeedProperty {
  tweets: Array<Tweet>;
  tweetLikes: Record<string, number>;
  tweetIsLiked: Record<string, boolean>;
  onLike?: (tweetId: number) => () => void;
  onAuthorClick?: (author: string) => () => void;
  onTransfer?: (tweetId: number) => () => void;
  onCollect?: (tweetId: number) => () => void;
}

const Feed = ({
  tweets,
  tweetLikes,
  tweetIsLiked,
  onLike,
  onAuthorClick,
  onTransfer,
  onCollect,
}: FeedProperty) => {
  return (
    <div id="feed">
      {tweets.map((tweet) => {
        return (
          <TweetComponent
            key={tweet.id}
            tweet={tweet}
            likes={tweetLikes[tweet.id]}
            isLiked={tweetIsLiked[tweet.id]}
            onLike={onLike && onLike(tweet.id)}
            onAuthorClick={onAuthorClick && onAuthorClick(tweet.author)}
            onTransfer={onTransfer && onTransfer(tweet.id)}
            onCollect={onCollect && onCollect(tweet.id)}
          />
        );
      })}
    </div>
  );
};

export default Feed;
