import { Tweet } from '../lib/tweet';
import './TweetComponent.css';

interface TweetProperty {
  tweet: Tweet;
  likes: number,
  isLiked: boolean,
  onLike?: () => void;
  onTransfer?: () => void;
  onAuthorClick?: () => void;
  onCollect?: () => void;
}

const TweetComponent = ({
  tweet,
  likes,
  isLiked,
  onLike,
  onAuthorClick,
  onTransfer,
  onCollect,
}: TweetProperty) => {
  const { id, author, content } = tweet;
  const authorClassNames: string = [
    'tweet-author',
    ...(onAuthorClick ? ['clickable-tweet-author'] : []),
  ].join(' ');

  const isCollected = !!tweet.collected;

  const mintableTime = tweet.collected?.mintableDate.toLocaleTimeString(undefined, {
    hour: 'numeric',
    minute: 'numeric',
  });


  return (
    <div className={'tweet'}>
      <div className="tweet-header">
        <div className={authorClassNames} onClick={onAuthorClick}>
          {author}
        </div>
        <span className="tweet-header-separator">·</span>
        <div className="tweet-id">{id}</div>
      </div>
      {content.type === "text" ? <div className="tweet-content">{content.data}</div> : null}
      {content.type === "image" ? <div className="tweet-content image-container"><img className="content-image" src={content.data} /></div> : null}
      <div className="tweet-footer">
        {onLike && (
          <button
            className={'tweet-footer-buttom tweet-likes'}
            onClick={onLike}
            disabled={isCollected || isLiked}
          >
            <img
              className="tweet-footer-icon tweet-footer-button-with-text"
              src={isLiked ? '/heart-fill.svg' : '/heart.svg'}
              alt="heart"
            />
            <span>{likes}</span>
          </button>
        )}
        {onTransfer && !isCollected && (
          <button
            className={'tweet-footer-buttom tweet-transfer'}
            onClick={onTransfer}
          >
            <img
              className="tweet-footer-icon"
              src={'/transfer.svg'}
              alt="transfer"
            />
          </button>
        )}
        {onCollect && !isCollected && (
          <button
            className={'tweet-footer-buttom tweet-collect'}
            onClick={onCollect}
          >
            <img
              className="tweet-footer-icon"
              src={'/collect.svg'}
              alt="collect"
            />
          </button>
        )}
        {tweet.collected && (
          <button className={'tweet-footer-buttom tweet-mint'} disabled={true}>
            <img
              className="tweet-footer-icon tweet-footer-button-with-text"
              src={'/tezos.svg'}
              alt="tezos"
            />
            <span>
              {new Date() > tweet.collected.mintableDate
                ? 'Mintable now!'
                : 'Mintable on ' +
                tweet.collected.mintableDate.toLocaleDateString() +
                ' at ' +
                mintableTime
              }
            </span>
          </button>
        )}
      </div>
    </div>
  );
};

export default TweetComponent;
