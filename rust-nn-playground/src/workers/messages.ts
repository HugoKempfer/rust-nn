import { MnistImage } from "@/models/MnistDatasetModel";
import { Network } from "rust-nn";

export enum MessageType {
  TRAIN_NETWORK,
  TRAIN_SUCCESS,
  TRAIN_UPDATE
}

export interface TrainSuccess {
  network: string;
  correctPredictNb: number;
  errorPredictNb: number;
}

export interface TrainUpdate {
  message: string;
  incrImage: number;
}

export interface TrainNetwork {
  hiddenNb: number;
  learningRate: number;
  trainImages: MnistImage[];
  testImages: MnistImage[];
  updateEveryNImage: number;
}

export interface MnistMessage {
  type: MessageType;
  value: TrainNetwork | TrainSuccess | TrainUpdate | null;
}
