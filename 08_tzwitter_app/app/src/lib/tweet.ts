export interface CollectedData {
  level: number;
  mintableDate: Date;
}

export interface Tweet {
  id: number;
  author: string;
  content: Content;
  likes: number;
  isLiked: boolean;
  collected?: CollectedData;
}

export type Content = Image | Text;

export interface Image {
  type: "image",
  data: string
}
export interface Text {
  type: "text",
  data: string
}