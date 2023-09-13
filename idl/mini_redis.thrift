namespace rs mini.redis


enum RequestType{
  Get,
  Set,
  Del,
  Subscribe,
  Publish,
  Ping
}

struct Request{
  1: required RequestType request_type,
  2: optional string key,
  3: optional string value,
  4: optional i32 ttl
}
enum ResponseType{
  Ok,
  Error,
  Message
}
struct Response{
  1: required ResponseType response_type,
  2: optional string message
}
service ItemService{
  Response RedisCommand(1: Request request)
}