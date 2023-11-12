#!/usr/local/bin/python3
import leetcode
import sys
import json

DEFAULT_QUERY = "playground"
DEFAULT_VARIABLES = r'{"titleSlug":"two-sum"}'
DEFAULT_OPERATION_NAME = "getQuestionDetail"

def get_csrf_token(fname = "token"):
    with open(fname, "r") as t:
        return t.read().rstrip('\n')
def get_session(fname = "session"):
    with open(fname, "r") as s:
        return s.read().rstrip('\n')

configuration = leetcode.Configuration()
configuration.api_key['x-csrftoken'] = get_csrf_token()
configuration.api_key['csrftoken'] = configuration.api_key['x-csrftoken']
configuration.api_key['LEETCODE_SESSION'] = get_session()
configuration.api_key['Referer'] = "https://leetcode.com/"
configuration.debug = False

def query_argument():
    if len(sys.argv) == 1:
        return DEFAULT_QUERY
    else:
        return sys.argv[1]

def variables_argument():
    if len(sys.argv) < 3:
        return DEFAULT_VARIABLES
    elif len(sys.argv) < 4:
        return f'{{"titleSlug":"{sys.argv[2]}"}}'

def get_query(query_filename = DEFAULT_QUERY):
    with open(f"queries/{query_filename}.graphql", "r") as f:
        return f.read()

api_instance = leetcode.DefaultApi(leetcode.ApiClient(configuration))
request = leetcode.GraphqlQuery(
            query=get_query(query_argument()), 
            variables=variables_argument(),
        )

response_obj = api_instance.graphql_post(body=request)
response_json_str = json.dumps(response_obj.to_dict())
print(response_json_str)
# print(response_obj.to_str())


