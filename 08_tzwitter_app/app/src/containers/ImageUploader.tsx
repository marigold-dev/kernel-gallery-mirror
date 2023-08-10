import React from 'react';
import './ImageUploader.css';

// Base64 to Hex
function base64ToHex(str: string) {
    const raw = Buffer.from(str).toString('hex');
    return raw.toUpperCase();
}

interface ImageUploader {
    onImageLoaded: (base64String: string) => void;
}

const ImageUploaderProcess: React.FC<ImageUploader> = ({ onImageLoaded }) => {
    // handle button click
    const handleButtonClick = () => {
        const fileInput = document.createElement('input');
        fileInput.type = 'file';
        fileInput.accept = 'image/*';

        fileInput.addEventListener('change', (event) => {
            // Read file from the file system
            const file = (event.target as HTMLInputElement).files?.[0];

            if (file) {
                const reader = new FileReader();

                reader.onloadend = async () => {
                    // encode base64
                    const base64String = reader.result as string;
                    console.log('Base64 image:', base64String);
                    const hexaString = base64ToHex(base64String);
                    console.log('Hex image:', hexaString);
                    onImageLoaded(hexaString);
                };
                reader.readAsDataURL(file);
            }
        });
        fileInput.click();
    };
    return (
        <button id="submit" type="button" onClick={handleButtonClick}> Load image </button>
    );
};

export default ImageUploaderProcess;

