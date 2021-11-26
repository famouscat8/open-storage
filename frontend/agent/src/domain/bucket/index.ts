export type UploadChunkResponse =
    | "success"
    | "blob_already_exists"
    | "chunk_already_exists"
    | "allowance_reached"
    | "user_not_found"
    | "hash_mismatch"
    | "full";

export type DeleteBlobResponse =
    | "success"
    | "not_authorized"
    | "not_found";