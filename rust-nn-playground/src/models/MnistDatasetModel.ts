import { plainToClass } from "class-transformer";
import axios, { AxiosResponse } from "axios";

class MnistImage {
  constructor(label: number[], bitmap: number[]) {
    this.label = label;
    this.image = bitmap;
  }

  label: Array<number>;
  image: Array<number>;
}

export async function buildMnist(
  trainSize: number,
  testSize: number
): Promise<{ trainImages: Array<MnistImage>; testImages: Array<MnistImage> }> {
  const batchFileNb = 6;
  const trainImages: Array<MnistImage> = [];
  const testImages: Array<MnistImage> = [];

  loadNewFile: for (let fileNb = 0; fileNb <= batchFileNb; ++fileNb) {
    const file: AxiosResponse = await axios.get(
      `datasets/mnist/mnist_batch_${fileNb}.json`
    );
    const images = plainToClass(MnistImage, file.data.data as MnistImage[]);
    console.log(images.length);
    let fileIt = 0;
    while (trainImages.length < trainSize) {
      if (fileIt >= images.length) {
        continue loadNewFile;
      }
      trainImages.push(images[fileIt++]);
    }
    while (testImages.length < testSize) {
      if (fileIt >= images.length) {
        continue loadNewFile;
      }
      testImages.push(images[fileIt++]);
    }
    if (trainImages.length == trainSize && testImages.length == testSize) {
      break;
    }
  }
  if (trainImages.length != trainSize || testImages.length != testSize) {
    throw "The requested sizes are too large";
  }
  return { trainImages, testImages };
}
