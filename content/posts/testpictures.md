+++
title = "A TEST for Pictures"
date = "2026-06-10"
updated = "2026-06-10"
slug = "testpictures"
description = "A TEST for Pictures"
draft = false
+++

# 增加后台上传并使用图片的功能

之前使用的底层库 **axum** 默认对请求体大小有 2MB 的限制，因此超过 2MB 的图片就会被拦截并报出 `Error parsing multipart/form-data request` 。现在已经将默认限制提高到了 10MB，可以正常上传较大体积的截图或图片了。

这是测试的第一张图片，大小2MB，上传方式为手动上传：
<!-- 背景.png(images/背景.png) -->
> ! 目前已被删除

这是第二张图片，截屏后通过剪切板`Ctrl + v`上传：
![image.png](/images/image.png)

目前这个后台功能还是比较完善了。


