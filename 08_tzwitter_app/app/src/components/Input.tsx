import React from 'react';
import './Input.css';
import ImageUploader from '../containers/ImageUploader';

interface InputProperties {
  value: string;
  onChange: (e: React.ChangeEvent<HTMLInputElement>) => void;
  onSubmit: () => void;
  disabled: boolean;
  handleImageLoaded: (base64String: string) => void;
}

const Input: React.FC<InputProperties> = ({
  value,
  onChange,
  onSubmit,
  disabled,
  handleImageLoaded,
}) => {
  return (
    <div id="form">
      <input
        id="input"
        placeholder="What's happening?"
        onChange={onChange}
        value={value}
      ></input>
      <div className="button-container">
        <button id="submit" type="button">
          <ImageUploader onImageLoaded={handleImageLoaded} />
        </button>
        <button id="submit" onClick={onSubmit} disabled={disabled}>
          Tweet
        </button>
      </div>
    </div>
  );
};

export default Input;

