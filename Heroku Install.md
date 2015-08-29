## Specify a buildpack for a rust project

$> heroku create --buildpack https://github.com/emk/heroku-buildpack-rust.git
created app calm-plains-3817
$> heroku git:remote -a calm-plains-3817
$> git remote show
heroku
origin
$>git remote show heroku
* remote heroku
  Fetch URL: https://git.heroku.com/calm-plains-3817.git
  Push  URL: https://git.heroku.com/calm-plains-3817.git
  
$heroku apps:rename iron-curtain --app calm-plains-3817
Renaming calm-plains-3817 to iron-curtain... done
https://iron-curtain.herokuapp.com/ | https://git.heroku.com/iron-curtain.git
Git remote heroku updated

git remote show heroku
* remote heroku
  Fetch URL: https://git.heroku.com/iron-curtain.git
  Push  URL: https://git.heroku.com/iron-curtain.git
  
To switch to ssh git  
git remote set-url heroku git@heroku.com:iron-curtain.git

heroku apps:rename bazaar_db --app heroku-postgres-50ddc241