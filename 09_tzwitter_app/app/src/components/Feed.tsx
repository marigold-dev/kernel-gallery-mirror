import { Tweet } from '../lib/tweet';
import TweetComponent from './TweetComponent';
import './Feed.css';

interface FeedProperty {
  tweets: Array<Tweet>;
  onLike?: (tweetId: number) => () => void;
  onAuthorClick?: (author: string) => () => void;
  onTransfer?: (tweetId: number) => () => void;
  onCollect?: (tweetId: number) => () => void;
}

const Feed = ({
  tweets,
  onLike,
  onAuthorClick,
  onTransfer,
  onCollect,
}: FeedProperty) => {
  tweets.sort((tweetA, tweetB) => {
    return tweetB.id - tweetA.id;
  });

  return (
    <div id="feed">
      {tweets.map((tweet) => {
        return (
          <TweetComponent
            key={tweet.id}
            tweet={tweet}
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
