<?php declare(strict_types=1);

FFI::load(__DIR__ . "/bindings.h");
opcache_compile_file(__DIR__ . "/php/test.php");
