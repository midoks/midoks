/**
 * Welcome to Cloudflare Workers! This is your first worker.
 *
 * - Run "npm run dev" in your terminal to start a development server
 * - Open a browser tab at http://localhost:8787/ to see your worker in action
 * - Run "npm run deploy" to publish your worker
 *
 * Learn more at https://developers.cloudflare.com/workers/
 */

import url from 'url';
import { Buffer } from 'node:buffer';

export default {
    async fetch(request, env, ctx) {
        // console.log("test");

        const cache = caches.default;
        const reqUrl = url.parse(request.url);

        var image_url = reqUrl.pathname.substring(1);

        if (image_url != ''){
            try {
                if (image_url == "favicon.ico"){
                    return new Response('ok!');
                }

                image_url = decodeURIComponent(image_url);
                console.log('fetch pre:'+image_url);
                const buff = Buffer.from(image_url, 'base64');
                image_url = buff.toString('utf-8');
                
                console.log('fetch2:'+image_url);
                const cacheKey = new Request(image_url);

                const hasCache = await cache.match(cacheKey);
                if (hasCache) {
                    console.log('cache: hit:'+image_url);
                    return hasCache;
                }

                // 目标图片获取与检查
                const imageRes = await fetch(image_url, { headers: request.headers });
                if (!imageRes.ok) {
                    return imageRes;
                }
                console.log('fetch image done');
                console.log(imageRes);
                // const imageBytes = new Uint8Array(await imageRes.arrayBuffer());

                // 返回体构造
                const imageResponse = new Response(imageRes.body, {
                    headers: {
                        'cache-control': 'public,max-age=86400',
                    },
                });

                // 写入缓存
                ctx.waitUntil(cache.put(cacheKey, imageResponse.clone()));
                return imageResponse;

            } catch (error) {
                console.error('process:error', error.name, error.message, error);
                return new Response('error!');
            }
        }
        return new Response('Hello World!');
    },
};