# rust <-> php via ffi


```
cargo build
docker build -f php-ffi php

docker run --rm -w $PWD -v $PWD:$PWD php-ffi php php/test.php
```


<div style="color:red">test</div>
