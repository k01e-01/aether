import flask
from flask import Flask

app = Flask("aether")

users = [
    "john doe",
    "jane doe",
    "mr. butt"
]

@app.route("/")
def index():
    return flask.render_template("index.html", users=users)

@app.route("/users/list")
def partials_user_list():
    return flask.render_template("partials/user_list.html", users=users)

@app.route("/users/add", methods=["POST"])
def add_user():
    users.append(flask.request.form["add_user"])
    return flask.render_template("partials/user_list.html", users=users)


if __name__ == "__main__":
    app.run()
