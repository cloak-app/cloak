export const getNovelCover = (cover?: ArrayBuffer) => {
  if (!cover) return '';

  const int8Array = new Int8Array(cover);
  const buffer = int8Array.buffer;

  return URL.createObjectURL(new Blob([buffer]));
};

export const getNovelFileExtension = (path: string) => {
  return path.split('.').pop()?.toUpperCase();
};
