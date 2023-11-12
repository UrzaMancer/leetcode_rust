#!/bin/sh
# --data 'variables=%7B%22titleSlug%22%3A%20%22two-sum%22%7D' \
# --data 'variables={"titleSlug": "two-sum"}'

curl -Gw "\n%{url}\n" --location 'https://leetcode.com/graphql/' \
--header 'Cookie: LEETCODE_SESSION=@session' \
--header 'Cookie: csrftoken=@token' \
--header 'Cookie: x-csrftoken=@token' \
--header 'Referer: https://leetcode.com/' \
--data-urlencode query@queries/get_question_detail.graphql \
--data-urlencode 'variables={"titleSlug": "two-sum"}'
