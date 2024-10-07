# stock-handbook

## How to run
```shell
$ systemctl stop firewalld
$ setenforce 0
$ sh backend/env.sh
$ cd backend
$ cargo run
$ cd frontend
$ npm run serve
```
Go to MongoDB management page > Database > Clusters > Connect > Add current IP 
