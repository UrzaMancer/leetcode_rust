#!/bin/sh

curl -Gw "\n%{url}\n" --location 'https://leetcode.com/graphql/' \
--header 'Cookie: LEETCODE_SESSION=@session' \
--header 'Cookie: csrftoken=@token' \
--header 'Cookie: x-csrftoken=@token' \
--header 'Referer: https://leetcode.com/' \
--data-urlencode query@queries/get_question_detail.graphql \
--data-urlencode 'variables={"titleSlug": "sort-vowels-in-a-string"}'
