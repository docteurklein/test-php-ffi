<?php declare(strict_types=1);

//$ffi = FFI::scope('TESTPHP');
$ffi = FFI::load(__DIR__.'/../bindings.h');
$point = $ffi->new('Point');
$point->x = 2;
$point->y = 2;
//var_dump($point);
$ffi->hello_from_rust($point);
