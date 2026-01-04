# Assignment : Movie Database SQL Queries

This document contains 50 SQL queries for a movie database management system. Each query includes the question, SQL solution, and Output format.


## Query 1

**Question:** Write a SQL query to find the name and year of the movies. Return movie title, movie release year.

**SQL Query:**

```sql
select mov_title,mov_year from movie;
```

**Output:**
| mov_title | mov_year |
|-----------------------------|------------------|
| Vertigo | 1958 |
| The Innocents | 1961 |
| Lawrence of Arabia | 1962 |
| The Deer Hunter | 1978 |
| Amadeus | 1984 |
| Blade Runner | 1982 |
| Eyes Wide Shut | 1999 |
| The Usual Suspects | 1995 |
| Chinatown | 1974 |
| Boogie Nights | 1997 |
| Annie Hall | 1977 |
| Princess Mononoke | 1997 |
| The Shawshank Redemption | 1994 |
| American Beauty | 1999 |
| Titanic | 1997 |
| Good Will Hunting | 1997 |
| Deliverance | 1972 |
| Trainspotting | 1996 |
| The Prestige | 2006 |
| Donnie Darko | 2001 |
| Slumdog Millionaire | 2008 |
| Aliens | 1986 |
| Beyond the Sea | 2004 |
| Avatar | 2009 |
| Seven Samurai | 1954 |
| Spirited Away | 2001 |
| Back to the Future | 1985 |
| Braveheart | 1995 |

---

## Query 2

**Question:** Write a SQL query to find when the movie 'American Beauty' released. Return movie release year.

**SQL Query:**

```sql
select mov_year from movie where mov_title = 'American Beauty';
```

**Output:**
| mov_year |
|-------------------|
| 1999 |

---

## Query 3

**Question:** Write a SQL query to find the movie that was released in 1999. Return movie title.

**SQL Query:**

```sql
select mov_title from movie where mov_year = 1999;
```

**Output:**
| movie_title |
|-------------|
Eyes Wide Shut
American Beauty

---

## Query 4

**Question:** Write a SQL query to find those movies, which were released before 1998. Return movie title.

**SQL Query:**

```sql
select mov_title from movie where mov_year < 1998;
```

**Output:**
| mov_title |
|-------------|
Vertigo
The Innocents
Lawrence of Arabia
The Deer Hunter
Amadeus
Blade Runner
The Usual Suspects
Chinatown
Boogie Nights
Annie Hall
Princess Mononoke
The Shawshank Redemption
Titanic
Good Will Hunting
Deliverance
Trainspotting
Aliens
Seven Samurai
Back to the Future
Braveheart

---

## Query 5

**Question:** Write a SQL query to find the name of all reviewers and movies together in a single list.

**SQL Query:**

```sql
select rev_name AS REVIEWR_MOVIE from MOVIE_REVIEWER UNION select mov_title from MOVIE;
```

**Output:**
| REVIEWR_MOVIE |
|------|
Righty Sock
Jack Malvern
Flagrant Baronessa
Alec Shaw
null
Victor Woeltjen
Simon Wright
Neal Wruck
Paul Monks
Mike Salvati
Wesley S. Walker
Sasha Goldshtein
Josh Cates
Krug Stillo
Scott LeBrun
Hannah Steele
Vincent Cadena
Brandt Sponseller
Richard Adams
Vertigo
The Innocents
Lawrence of Arabia
The Deer Hunter
Amadeus
Blade Runner
Eyes Wide Shut
The Usual Suspects
Chinatown
Boogie Nights
Annie Hall
Princess Mononoke
The Shawshank Redemption
American Beauty
Titanic
Good Will Hunting
Deliverance
Trainspotting
The Prestige
Donnie Darko
Slumdog Millionaire
Aliens
Beyond the Sea
Avatar
Seven Samurai
Spirited Away
Back to the Future
Braveheart

---

## Query 6

**Question:** Write a SQL query to find all reviewers who have rated seven or more stars to their rating. Return reviewer name.

**SQL Query:**

```sql
select DISTINCT mr.rev_name from MOVIE_REVIEWER mr JOIN movie_rating rt on mr.rev_id = rt.rev_id where rt.rev_stars>=7; 
```

**Output:**
| rev_name |
|---------------------|
| Righty Sock |
| Jack Malvern |
| Flagrant Baronessa |
| (null) |
| Simon Wright |
| Mike Salvati |
| Sasha Goldshtein |
| Righty Sock |
| Hannah Steele |
| Vincent Cadena |
| Brandt Sponseller |
| Victor Woeltjen |
| Krug Stillo |

---

## Query 7

**Question:** Write a SQL query to find the movies without any rating. Return movie title.

**SQL Query:**

```sql
select mv.MOV_TITLE from MOVIE mv JOIN MOVIE_RATING mr on mv.mov_id = mr.mov_id where mr.NUM_O_RATINGS IS NULL;
```

**Output:**
| mov_title |
|-------------|
Princess Mononoke
Avatar

---

## Query 8

**Question:** Write a SQL query to find the movies with ID 905 or 907 or 917. Return movie title.

**SQL Query:**

```sql
select mov_title from movie mv where mv.MOV_ID = 905 OR mv.mov_ID = 907 or mv.mov_id = 917; 
```

**Output:**
| mov_name |
|------------|

---

## Query 9

**Question:** Write a SQL query to find the movie titles that contain the word 'Boogie Nights'. Sort the result-set in ascending order by movie year. Return movie ID, movie title and movie release year.

**SQL Query:**

```sql
select mv.mov_id, mv.mov_title, mv.mov_year from movie mv where mv.mov_title LIKE '%Boogie Nights%' ORDER by MOV_YEAR 
```

**Output:**
| mov_ID | mov_title | mov_year |
|----------|-------------|-------------------|
| 10 | Boogie Nights | 1997 |

---

## Query 10

**Question:** Write a SQL query to find those actors with the first name 'Woody' and the last name 'Allen'. Return actor ID.

**SQL Query:**

```sql
select act_id from actor where act_fname = 'Woody' AND acrt_lname = 'Allen';
```

**Output:**
| act_ID |
|----------|
| 11 |

---

# Sub-Queries:

## Query 11

**Question:** Write a SQL query to find the actors who played a role in the movie 'Annie Hall'. Return all the fields of actor table.

**SQL Query:**

```sql
select * from actor where act_id in 
(select act_id from movie_cast where mov_id in 
(select mov_id from movie where mov_title = 'Annie Hall'));
```

**Output:**
| act_id | act_fname | acrt_lname | act_gender |
|--------|-----------|------------|------------|
| 11 | Woody | Allen | M |

---

## Query 12

**Question:** Write a SQL query to find the director of a film that cast a role in 'Eyes Wide Shut'. Return director first name, last name.

**SQL Query:**

```sql
select dir_fname , dir_lname from director where dir_id in 
(select dir_id from movie_direction where mov_id in
(select mov_id from movie where mov_title = 'Eyes Wide Shut'));
```

**Output:**
| dir_first_name | dir_last_name |
|---------------------|-------------------|
| Stanley | Kubrick |

---

## Query 13

**Question:** Write a SQL query to find those movies that have been released in countries other than the United Kingdom. Return movie title, movie year, movie time, and date of release, releasing country.

**SQL Query:**

```sql
select mov_title, mov_year, mov_time, mov_dt_rel, mov_rel_country from movie where mov_rel_country <> 'UK';
```

**Output:**
| mov_title | mov_year | mov_time | mov_dt_rel | mov_rel_country |
|-----------------|----------|----------|------------|----------------|
| The Innocents | 1961 | 100 | 1962-02-19 | SW |
| Annie Hall | 1977 | 93 | 1977-04-20 | USA |
| Seven Samurai | 1954 | 207 | 1954-04-26 | JP |

---

## Query 14

**Question:** Write a SQL query to find for movies whose reviewer is unknown. Return movie title, year, release date, director first name, last name, actor first name, last name.

**SQL Query:**

```sql
SELECT
    mov_title,
    mov_year,
    mov_dt_rel,
    (SELECT dir_fname
     FROM director
     WHERE dir_id = (SELECT dir_id
                     FROM movie_direction
                     WHERE mov_id = m.mov_id)) AS director_fname,
    (SELECT dir_lname
     FROM director
     WHERE dir_id = (SELECT dir_id
                     FROM movie_direction
                     WHERE mov_id = m.mov_id)) AS director_lname,
    (SELECT act_fname
     FROM actor
     WHERE act_id = (SELECT act_id
                     FROM movie_cast
                     WHERE mov_id = m.mov_id)) AS actor_fname,
    (SELECT acrt_lname
     FROM actor
     WHERE act_id = (SELECT act_id
                     FROM movie_cast
                     WHERE mov_id = m.mov_id)) AS actor_lname
FROM movie m
WHERE
	mov_id
	IN
(SELECT mov_id
 FROM movie_rating
 WHERE rev_id
	IN
    (SELECT rev_id
	 FROM movie_reviewer
	 WHERE rev_name = '')
);
```

**Output:**
| mov_title | mov_year | mov_dt_rel | director_fname | director_lname | actor_fname | actor_lname |
|-------------------|----------|------------|----------------|----------------|-------------|-------------|
| Blade Runner | 1982 | 1982-09-09 | Ridley | Scott | Harrison | Ford |
| Princess Mononoke | 1997 | 2001-10-19 | Hayao | Miyazaki | Claire | Danes |

---

## Query 15

**Question:** Write a SQL query to find those movies directed by the director whose first name is Woddy and last name is Allen. Return movie title.

**SQL Query:**

```sql
select mov_title from movie where mov_id in (
    select mov_id from movie_direction where dir_id in(
        select dir_id from director where dir_fname = 'Woody' AND dir_lname = 'Allen'
    )
)
```

**Output:**
| mov_title |
|-------------|
| Annie Hall |

---

## Query 16

**Question:** Write a SQL query to determine those years in which there was at least one movie that received a rating of at least three stars. Sort the result-set in ascending order by movie year. Return movie year.

**SQL Query:**

```sql
select mov_year from movie where mov_id in (
    select mov_id from movie_rating where rev_stars >= 3  
) order by mov_year;
```

**Output:**
| mov_year |
|------------|
1954
1958
1961
1962
1977
1982
1986
1995
1997
1997
1997
1997
1999
2001
2004
2008
2009

---

## Query 17

**Question:** Write a SQL query to search for movies that do not have any ratings. Return movie title.

**SQL Query:**

```sql
select mov_title from movie where mov_id in (
    select mov_id from movie_rating where rev_stars is null
);
```

**Output:**
| mov_title |
|-------------|
Chinatown
Trainspotting

---

## Query 18

**Question:** Write a SQL query to find those reviewers who have not given a rating to certain films. Return reviewer name.

**SQL Query:**

```sql
select rev_name from movie_reviewer where rev_id  not in(
    select rev_id from movie_rating
);
```

**Output:**
| rev_name |
|---------------|
reviewer_name
Alec Shaw
Wesley S. Walker

---

## Query 19

**Question:** Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Sort the result-set in ascending order by reviewer name, movie title, review Stars. Return reviewer name, movie title, review Stars.

**SQL Query:**

```sql
SELECT
    (SELECT mr.rev_name
     FROM movie_reviewer mr
     WHERE mr.rev_id = r.rev_id) AS rev_name,

    (SELECT m.mov_title
     FROM movie m
     WHERE m.mov_id = r.mov_id) AS mov_title,

    r.rev_stars
FROM movie_rating r
WHERE r.rev_stars IS NOT NULL
ORDER BY
    rev_name ASC,
    mov_title ASC,
    r.rev_stars ASC;
```

**Output:**
| reviewer_name | movie_title | review_stars |
|-----------------------|------------------------|--------------|
| null | Blade Runner | 8.2 |
| null | Princess Mononoke | 8.4 |
| Brandt Sponseller | Aliens | 8.4 |
| Flagrant Baronessa | Lawrence of Arabia | 8.3 |
| Hannah Steele | Donnie Darko | 8.1 |
| Jack Malvern | The Innocents | 7.9 |
| Josh Cates | Good Will Hunting | 4.0 |
| Krug Stillo | Seven Samurai | 7.7 |
| Mike Salvati | Annie Hall | 8.1 |
| Paul Monks | Boogie Nights | 3.0 |
| Richard Adams | Beyond the Sea | 6.7 |
| Righty Sock | Titanic | 7.7 |
| Righty Sock | Vertigo | 8.4 |
| Sasha Goldshtein | American Beauty | 7.0 |
| Simon Wright | The Usual Suspects | 8.6 |
| Victor Woeltjen | Avatar | 7.3 |
| Vincent Cadena | Slumdog Millionaire | 8.0 |


---

## Query 20

**Question:** Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Group the result set on reviewer’s name, movie title. Return reviewer’s name, movie title.

**SQL Query:**

```sql
SELECT DISTINCT
    (SELECT mr.rev_name
     FROM movie_reviewer mr
     WHERE mr.rev_id = r.rev_id) AS rev_name,

    (SELECT m.mov_title
     FROM movie m
     WHERE m.mov_id = r.mov_id) AS mov_title
FROM movie_rating r
WHERE r.rev_stars IS NOT NULL;
```

**Output:**
| reviewer_name | movie_title |
|-----------------------|------------------------|
| Righty Sock | Vertigo |
| Jack Malvern | The Innocents |
| Flagrant Baronessa | Lawrence of Arabia |
| null | Blade Runner |
| Simon Wright | The Usual Suspects |
| Paul Monks | Boogie Nights |
| Mike Salvati | Annie Hall |
| null | Princess Mononoke |
| Sasha Goldshtein | American Beauty |
| Righty Sock | Titanic |
| Josh Cates | Good Will Hunting |
| Hannah Steele | Donnie Darko |
| Vincent Cadena | Slumdog Millionaire |
| Brandt Sponseller | Aliens |
| Richard Adams | Beyond the Sea |
| Victor Woeltjen | Avatar |
| Krug Stillo | Seven Samurai |


---

## Query 21

**Question:** Write a SQL query to find those movies, which have received highest number of stars. Group the result set on movie title and sorts the result-set in ascending order by movie title. Return movie title and maximum number of review stars.

**SQL Query:**

```sql
select m.mov_title,
       max(r.rev_stars) AS max_stars
from movie m, movie_rating r
where m.mov_id = r.mov_id
  and r.rev_stars = (
        select MAX(rev_stars)
        from movie_rating
        where rev_stars IS NOT NULL
      )
GROUP BY m.mov_title
ORDER BY m.mov_title ASC;
```

**Output:**
| mov_title | max_stars |
|-----------|-----------|
| The Usual Suspects| 8.6|

---

## Query 22

**Question:** Write a SQL query to find all reviewers who rated the movie 'American Beauty'. Return reviewer name.

**SQL Query:**

```sql
select rev_name from movie_reviewer where rev_id in(
    select rev_id from movie_rating where mov_id in(
        select mov_id from movie where mov_title  = 'American Beauty'
    )
);
```

**Output:**
| rev_name |
|---------------|
Sasha Goldshtein

---

## Query 23

**Question:** Write a SQL query to find the movies that have not been reviewed by any reviewer body other than 'Paul Monks'. Return movie title.

**SQL Query:**

```sql
select mov_title from movie where mov_id in (
    select mov_id from movie_rating where rev_id in (
        select rev_id from movie_reviewer where rev_name = 'Paul Monks'
    )
);
```

**Output:**
| mov_title |
|-------------|
Boogie Nights

---

## Query 24

**Question:** Write a SQL query to find the movies with the lowest ratings. Return reviewer name, movie title, and number of stars for those movies.

**SQL Query:**

```sql
SELECT
    (SELECT mr.rev_name
     FROM movie_reviewer mr
     WHERE mr.rev_id = r.rev_id) AS reviewer_name,

    (SELECT m.mov_title
     FROM movie m
     WHERE m.mov_id = r.mov_id) AS movie_title,

    r.rev_stars
FROM movie_rating r
WHERE r.rev_stars = (
        SELECT MIN(rev_stars)
        FROM movie_rating
        WHERE rev_stars IS NOT NULL
      );
```

**Output:**
| rev_name | mov_title | rev_stars |
|---------------|-------------|--------------|
| Paul Monks |Boogie Nights| 3|

---

## Query 25

**Question:** Write a SQL query to find the movies directed by 'James Cameron'. Return movie title.

**SQL Query:**

```sql
select mov_title from movie where mov_id in (
    select mov_id from movie_direction where dir_id in (
        select dir_id from director where dir_fname = 'James' AND dir_lname = 'Cameron'
    )
);
```

**Output:**
| mov_title |
|-------------|
Titanic
Aliens

---

## Query 26

**Question:** Write a query in SQL to find the movies in which one or more actors appeared in more than one film.

**SQL Query:**

```sql
SELECT DISTINCT m.mov_title
FROM movie m
WHERE m.mov_id IN (
    SELECT mc.mov_id
    FROM movie_cast mc
    WHERE mc.act_id IN (
        SELECT act_id
        FROM movie_cast
        GROUP BY act_id
        HAVING COUNT(DISTINCT mov_id) > 1
    )
);
```

**Output:**
| mov_title |
|-----------|
American Beauty
Beyond the Sea

---

## Query 27

**Question:** Write a SQL query to find all reviewers whose ratings contain a NULL value. Return reviewer name.

**SQL Query:**

```sql
select rev_name from movie_reviewer mr join movie_rating mrt on mr.rev_id = mrt.rev_id where rev_stars is null;
```

**Output:**
| rev_name |
|---------------|
Neal Wruck
Scott LeBrun

---

## Query 28

**Question:** Write a SQL query to find out who was cast in the movie 'Annie Hall'. Return actor first name, last name and role.

**SQL Query:**

```sql
SELECT
    a.act_fname,
    a.acrt_lname,
    mc.role
FROM movie m
JOIN movie_cast mc
    ON m.mov_id = mc.mov_id
JOIN actor a
    ON mc.act_id = a.act_id
WHERE m.mov_title = 'Annie Hall';
```

**Output:**
| act_fname | acrt_lname | role |
|------------|-----------|------|
| Woody | Allen | Alvy Singer |

---

## Query 29

**Question:** Write a SQL query to find the director who directed a movie that featured a role in 'Eyes Wide Shut'. Return director first name, last name and movie title.

**SQL Query:**

```sql
select dr.dir_fname, dr.dir_lname, mv.mov_title from movie mv join movie_direction md 
    on mv.mov_id = md.mov_id join director dr on md.dir_id =dr.dir_id where mv.mov_title = 'Eyes Wide Shut'; 
```

**Output:**
| dir_fname | dir_lname | mov_title |
|-----------|-----------|-------------|
| Stanley | Kubrick | Eyes Wide Shut |

---

## Query 30

**Question:** Write a SQL query to find the director of a movie that cast a role as Sean Maguire. Return director first name, last name and movie title.

**SQL Query:**

```sql
select dr.dir_fname, dr.dir_lname, mv.mov_title from movie_direction md join movie mv 
on md.mov_id = mv.mov_id join director dr on md.dir_id = dr.dir_id join movie_cast mc 
on mv.mov_id = mc.mov_id where mc.role = 'Sean Maguire';
```

**Output:**
| dir_fname | dir_lname | mov_title |
|-----------|-----------|-------------|
| Gus | Van Sant | Good Will Hunting |

---

## Query 31

**Question:** Write a SQL query to find out which actors have not appeared in any movies between 1990 and 2000 (Begin and end values are included.). Return actor first name, last name, movie title and release year.

**SQL Query:**

```sql
select ac.act_fname, ac.acrt_lname, mv.mov_title, mv.mov_year from movie mv join movie_cast mc
on mv.mov_id = mc.mov_id join actor ac on ac.act_id = mc.act_id where mv.mov_year between 1990 and 2000
```

**Output:**
| act_fname | acrt_lname | mov_title | mov_year |
|----------------|-----------------|----------------------|----------|
| James | Stewart | Vertigo | 1958 |
| Deborah | Kerr | The Innocents | 1961 |
| Peter | OToole | Lawrence of Arabia | 1962 |
| Robert | De Niro | The Deer Hunter | 1978 |
| F. Murray | Abraham | Amadeus | 1984 |
| Harrison | Ford | Blade Runner | 1982 |
| Jack | Nicholson | Chinatown | 1974 |
| Christian | Bale | Chinatown | 1974 |
| Woody | Allen | Annie Hall | 1977 |
| Jon | Voight | Deliverance | 1972 |
| Maggie | Gyllenhaal | Donnie Darko | 2001 |
| Dev | Patel | Slumdog Millionaire | 2008 |
| Sigourney | Weaver | Aliens | 1986 |

---

## Query 32

**Question:** Write a SQL query to find the directors who have directed films in a variety of genres. Group the result set on director first name, last name and generic title. Sort the result-set in ascending order by director first name and last name. Return director first name, last name and number of genres movies.

**SQL Query:**

```sql
SELECT
    d.dir_fname,
    d.dir_lname,
    COUNT(DISTINCT g.gen_id) AS number_of_genres
FROM director d
JOIN movie_direction md
    ON d.dir_id = md.dir_id
JOIN movie_genres mg
    ON md.mov_id = mg.mov_id
JOIN genres g
    ON mg.gen_id = g.gen_id
GROUP BY
    d.dir_fname,
    d.dir_lname
HAVING COUNT(DISTINCT g.gen_title) > 1
ORDER BY
    d.dir_fname ASC,
    d.dir_lname ASC;
```

**Output:**
| dir_fname | dir_lname | number_of_genres |
|-----------|-----------|------------------|

---

## Query 33

**Question:** Write a SQL query to find the movies with year and genres. Return movie title, movie year and generic title.

**SQL Query:**

```sql
select mv.mov_year , gn.gen_title from movie mv join movie_genres mg on 
mv.mov_id = mg.mov_id join genres gn on gn.gen_id = mg.gen_id
```

**Output:**

| MOV_TITLE                | MOV_YEAR | GEN_TITLE |
|--------------------------|----------|-----------|
| Vertigo                  | 1958     | Mystery   |
| The Innocents            | 1961     | Horror    |
| Lawrence of Arabia       | 1962     | Adventure |
| The Deer Hunter          | 1978     | War       |
| Blade Runner             | 1982     | Thriller  |
| Eyes Wide Shut           | 1999     | Mystery   |
| The Usual Suspects       | 1995     | Crime     |
| Annie Hall               | 1977     | Comedy    |
| Princess Mononoke        | 1997     | Animation |
| The Shawshank Redemption | 1994     | Crime     |
| American Beauty          | 1999     | Romance   |
| Deliverance              | 1972     | Adventure |
| Trainspotting            | 1996     | Drama     |
| Slumdog Millionaire      | 2008     | Drama     |
| Aliens                   | 1986     | Action    |
| Beyond the Sea           | 2004     | Music     |
| Spirited Away            | 2001     | Drama     |
| Back to the Future       | 1985     | Mystery   |
| Braveheart               | 1995     | Drama     |


---

## Query 34

**Question:** Write a SQL query to find all the movies with year, genres, and name of the director.

**SQL Query:**

```sql
select mv.mov_year,gn.gen_title,dr.dir_fname from movie mv join movie_direction md 
on mv.mov_id = md.mov_id join director dr on md.dir_id = dr.dir_id join movie_genres mg on
mg.mov_id = mv.mov_id join genres gn on mg.gen_id = gn.gen_id;
```

**Output:**
### Output

| MOV_YEAR | GEN_TITLE  | DIR_FNAME |
|----------|------------|-----------|
| 1958 | Mystery   | Alfred |
| 1961 | Horror    | Jack   |
| 1962 | Adventure | David  |
| 1978 | War       | Michael |
| 1982 | Thriller  | Ridley |
| 1999 | Mystery   | Stanley |
| 1995 | Crime     | Bryan  |
| 1977 | Comedy    | Woody  |
| 1997 | Animation | Hayao  |
| 1994 | Crime     | Frank  |
| 1999 | Romance   | Sam    |
| 1986 | Action    | James  |
| 1972 | Adventure | John   |
| 1996 | Drama     | Danny  |
| 2008 | Drama     | Danny  |
| 2004 | Music     | Kevin  |


---

## Query 35

**Question:** Write a SQL query to find the movies released before 1st January 1989. Sort the result-set in descending order by date of release. Return movie title, release year, date of release, duration, and first and last name of the director.

**SQL Query:**

```sql
select mv.mov_title, mv.mov_year, mv.mov_dt_rel, mv.mov_time, dr.dir_fname,dr.dir_lname from
movie mv join movie_direction md on mv.mov_id = md.mov_id join director dr
on dr.dir_id = md.dir_id where mv.mov_dt_rel < DATE '1989-01-01' order by mv.mov_dt_rel desc;
```

**Output:**
| mov_title | mov_year | mov_dt_rel | mov_time | dir_fname | dir_lname |
|--------------------|----------|------------|----------|-----------|-----------|
| Aliens | 1986 | 1986-08-29 | 137 | James | Cameron |
| Amadeus | 1984 | 1985-01-07 | 160 | Milos | Forman |
| Deliverance | 1972 | 1982-10-05 | 109 | John | Boorman |
| Blade Runner | 1982 | 1982-09-09 | 117 | Ridley | Scott |
| The Deer Hunter | 1978 | 1979-03-08 | 183 | Michael | Cimino |
| Annie Hall | 1977 | 1977-04-20 | 93 | Woody | Allen |
| Chinatown | 1974 | 1974-08-09 | 130 | Roman | Polanski |
| Lawrence of Arabia | 1962 | 1962-12-11 | 216 | David | Lean |
| The Innocents | 1961 | 1962-02-19 | 100 | Jack | Clayton |
| Vertigo | 1958 | 1958-08-24 | 128 | Alfred | Hitchcock |

---

## Query 36

**Question:** Write a SQL query to calculate the average movie length and count the number of movies in each genre. Return genre title, average time and number of movies for each genre.

**SQL Query:**

```sql
select gn.gen_title, AVG(mv.mov_time), COUNT(mg.mov_id) from movie mv join movie_genres mg on 
mv.mov_id = mg.mov_id join genres gn on gn.gen_id = mg.gen_id group by gn.gen_title
```

**Output:**
### Output

| GEN_TITLE  | AVG(MV.MOV_TIME) | COUNT(MG.MOV_ID) |
|------------|------------------|------------------|
| Mystery    | 134.33333333333334 | 3 |
| Horror     | 100               | 1 |
| Adventure  | 162.5             | 2 |
| War        | 183               | 1 |
| Thriller   | 117               | 1 |
| Crime      | 124               | 2 |
| Comedy     | 93                | 1 |
| Animation  | 134               | 1 |
| Romance    | 122               | 1 |
| Drama      | 129.25            | 4 |
| Action     | 137               | 1 |
| Music      | 118               | 1 |


---

## Query 37

**Question:** Write a SQL query to find movies with the shortest duration. Return movie title, movie year, director first name, last name, actor first name, last name and role.

**SQL Query:**

```sql
SELECT mv.mov_title , mv.mov_year , dr.dir_fname,dr.dir_lname , ac.act_fname, ac.acrt_lname , mc.role
FROM 
movie mv JOIN movie_cast mc ON mc.mov_id = mv.mov_id
		JOIN actor ac ON ac.act_id = mc.act_id
        JOIN movie_direction md ON md.mov_id = mv.mov_id
        JOIN director dr ON dr.dir_id = md.dir_id
WHERE 
	mv.mov_time <= ALL(SELECT mov_time FROM movie);
```

**Output:**
| mov_title | mov_year | dir_fname | dir_lname | act_fname | acrt_lname | role |
|-----------|----------|-----------|-----------|-----------|------------|------|
| Annie Hall| 1977| Woody| Allen| Woody| Allen| Alvy Singer|

---

## Query 38

**Question:** Write a SQL query to find the years in which a movie received a rating of 3 or 4. Sort the result in increasing order on movie year.

**SQL Query:**

```sql
SELECT mv.mov_year  FROM
movie mv JOIN movie_rating mrt ON mv.mov_id = mrt.mov_id 
WHERE mrt.rev_stars= 3 OR mrt.rev_stars = 4 
GROUP BY mv.mov_year
ORDER BY mv.mov_year;
```

**Output:**
| mov_year |
|------------|
|1997|

---

## Query 39

**Question:** Write a SQL query to get the reviewer name, movie title, and stars in an order that reviewer name will come first, then by movie title, and lastly by number of stars.

**SQL Query:**

```sql
SELECT mr.rev_name  , mv.mov_title  , mrt.rev_stars
FROM 
movie mv JOIN movie_rating mrt ON mv.mov_id = mrt.mov_id
		JOIN movie_reviewer mr ON mr.rev_id = mrt.rev_id
		ORDER BY mr.rev_name ASC,
				 mv.mov_title ASC,
                 mrt.rev_stars ASC;
```

**Output:**
### Output

| REV_NAME              | MOV_TITLE              | REV_STARS |
|-----------------------|------------------------|-----------|
| Brandt Sponseller     | Aliens                 | 8.4       |
| Flagrant Baronessa    | Lawrence of Arabia     | 8.3       |
| Hannah Steele         | Donnie Darko           | 8.1       |
| Jack Malvern          | The Innocents          | 7.9       |
| Josh Cates            | Good Will Hunting      | 4.0       |
| Krug Stillo           | Seven Samurai          | 7.7       |
| Mike Salvati          | Annie Hall             | 8.1       |
| Neal Wruck            | Chinatown              | NULL      |
| Paul Monks            | Boogie Nights          | 3.0       |
| Richard Adams         | Beyond the Sea         | 6.7       |
| Righty Sock           | Titanic                | 7.7       |
| Righty Sock           | Vertigo                | 8.4       |
| Sasha Goldshtein      | American Beauty        | 7.0       |
| Scott LeBrun          | Trainspotting          | NULL      |
| Simon Wright          | The Usual Suspects     | 8.6       |
| Victor Woeltjen       | Avatar                 | 7.3       |
| Vincent Cadena        | Slumdog Millionaire    | 8.0       |
| NULL                  | Blade Runner           | 8.2       |
| NULL                  | Princess Mononoke      | 8.4       |


---

## Query 40

**Question:** Write a SQL query to find those movies that have at least one rating and received the most stars. Sort the result-set on movie title. Return movie title and maximum review stars.

**SQL Query:**

```sql
SELECT 
    mv.mov_title,
    ac.act_fname,
    ac.acrt_lname,
    mc.role
FROM movie_cast mc
JOIN actor ac ON mc.act_id = ac.act_id
JOIN movie mv ON mc.mov_id = mv.mov_id
JOIN movie_cast mc2 ON mc.act_id = mc2.act_id
AND mc.mov_id <> mc2.mov_id
ORDER BY ac.act_fname, mv.mov_title;
```

**Output:**
| mov_title | max(mrt.rev_stars) |
|-----------|------------------|
|The Usual Suspects| 8.6|

---

## Query 41

**Question:** Write a SQL query to find out which movies have received ratings. Return movie title, director first name, director last name and review stars.

**SQL Query:**

```sql
SELECT mv.mov_title, dr.dir_fname , dr.dir_lname, mrt.rev_stars
FROM 
movie mv JOIN movie_rating mrt ON mv.mov_id = mrt.mov_id
		JOIN movie_direction md ON mv.mov_id = md.mov_id
        JOIN director dr ON dr.dir_id = md.dir_id
WHERE 
	mrt.num_o_ratings > 0;
```

**Output:**
| mov_name | dir_fname | dir_lname | rev_stars |
|-----------------------|-----------|-----------------|--------------|
| Vertigo | Alfred | Hitchcock | 8.4 |
| The Innocents | Jack | Clayton | 7.9 |
| Lawrence of Arabia | David | Lean | 8.3 |
| Blade Runner | Ridley | Scott | 8.2 |
| The Usual Suspects | Bryan | Singer | 8.6 |
| Chinatown | Roman | Polanski | NULL |
| Boogie Nights | Paul | Thomas Anderson | 3.0 |
| Annie Hall | Woody | Allen | 8.1 |
| American Beauty | Sam | Mendes | 7.0 |
| Titanic | James | Cameron | 7.7 |
| Good Will Hunting | Gus | Van Sant | 4.0 |
| Trainspotting | Danny | Boyle | NULL |
| Donnie Darko | Richard | Kelly | 8.1 |
| Slumdog Millionaire | Danny | Boyle | 8.0 |
| Aliens | James | Cameron | 8.4 |
| Beyond the Sea | Kevin | Spacey | 6.7 |

---

## Query 42

**Question:** Write a SQL query to find movies in which one or more actors have acted in more than one film. Return movie title, actor first and last name, and the role.

**SQL Query:**

```sql
SELECT 
    mv.mov_title,
    ac.act_fname,
    ac.acrt_lname,
    mc.role
FROM movie_cast mc
JOIN actor ac ON mc.act_id = ac.act_id
JOIN movie mv ON mc.mov_id = mv.mov_id
JOIN movie_cast mc2 ON mc.act_id = mc2.act_id
AND mc.mov_id <> mc2.mov_id
ORDER BY ac.act_fname, mv.mov_title;
```

**Output:**
| mov_title | act_fname | acrt_lname | role |
|-----------|-----------|------------|------|
|American| Beauty| Kevin| Spacey| Lester Burnham|
|Beyond the Sea| Kevin| Spacey| Bobby Darin|

---

## Query 43

**Question:** Write a SQL query to find the actor whose first name is 'Claire' and last name is 'Danes'. Return director first name, last name, movie title, actor first name and last name, role.

**SQL Query:**

```sql
SELECT dr.dir_fname, dr.dir_lname,mv.mov_title , ac.act_fname , ac.acrt_lname , mc.role
FROM 
actor ac JOIN movie_cast mc ON mc.act_id = ac.act_id
		JOIN movie mv ON mv.mov_id = mc.mov_id
        JOIN movie_direction md on md.mov_id = mc.mov_id
        JOIN director dr ON dr.dir_id = md.dir_id
WHERE ac.act_fname = 'Claire' AND ac.acrt_lname='Danes';
```

**Output:**
| dir_fname | dir_lname | mov_title | act_fname | acrt_lname | role |
|-----------|-----------|-----------|-----------|------------|------|
| Hayao |Miyazaki |Princess Mononoke |Claire |Danes |San|

---

## Query 44

**Question:** Write a SQL query to find for actors whose films have been directed by them. Return actor first name, last name, movie title and role.

**SQL Query:**

```sql
SELECT 
    ac.act_fname,
    ac.acrt_lname,
    mv.mov_title,
    mc.role
FROM actor ac
JOIN movie_cast mc ON ac.act_id = mc.act_id
JOIN movie mv ON mc.mov_id = mv.mov_id
JOIN movie_direction md ON mv.mov_id = md.mov_id
JOIN director dr ON md.dir_id = dr.dir_id
WHERE 
    ac.act_fname = dr.dir_fname
    AND ac.acrt_lname = dr.dir_lname;
```

**Output:**
| act_fname | acrt_lname | mov_title | role |
|-----------|------------|-----------|------|
| Woody | Allen | Annie Hall | Alvy Singer |
| Kevin| Spacey| Beyond the Sea| Bobby Darin|

---

## Query 45

**Question:** Write a SQL query to find the cast list of the movie ‘Chinatown’. Return first name, last name.

**SQL Query:**

```sql
SELECT ac.act_fname, ac.acrt_lname 
FROM 
actor ac JOIN movie_cast mc ON ac.act_id = mc.act_id
		JOIN movie mv ON mc.mov_id = mv.mov_id
WHERE mv.mov_title = 'Chinatown';
```

**Output:**
| act_fname | acrt_lname |
|-----------|------------|
| Jack | Nicholson |
| Christian| Bale|

---

## Query 46

**Question:** Write a SQL query to find those movies where actor’s first name is 'Harrison' and last name is 'Ford'. Return movie title.

**SQL Query:**

```sql
SELECT mv.mov_title
FROM 
movie mv JOIN movie_cast mc ON mv.mov_id = mc.mov_id
		JOIN actor ac ON ac.act_id = mc.act_id
WHERE 
	ac.act_fname='Harrison' AND ac.acrt_lname='Ford';
```

**Output:**
| mov_title |
|-------------|
Blade Runner

---

## Query 47

**Question:** Write a SQL query to find the highest-rated movies. Return movie title, movie year, review stars and releasing country.

**SQL Query:**

```sql
SELECT 
    mv.mov_title,
    mv.mov_year,
    mrt.rev_stars,
    mv.mov_rel_country
FROM movie mv
JOIN movie_rating mrt ON mv.mov_id = mrt.mov_id

WHERE mrt.rev_stars = (select max(rev_stars) from movie_rating);
```

**Output:**
| mov_title | mov_year | rev_stars | mov_rel_country |
|----------------------|----------|-----------|----------------|
| The Usual Suspects | 1995 | 8.6 | UK |

---

## Query 48

**Question:** Write a SQL query to find the highest-rated ‘Mystery Movies’. Return the title, year, and rating.

**SQL Query:**

```sql
SELECT 
    mv.mov_title,
    mv.mov_year,
    mrt.rev_stars
FROM movie mv
JOIN movie_rating mrt 
    ON mv.mov_id = mrt.mov_id
JOIN movie_genres mg 
    ON mg.mov_id = mv.mov_id
JOIN genres gn 
    ON mg.gen_id = gn.gen_id
WHERE gn.gen_title = 'Mystery'
GROUP BY mv.mov_title, mv.mov_year, mrt.rev_stars
HAVING mrt.rev_stars = MAX(mrt.rev_stars);
```

**Output:**
| mov_title | mov_year | rev_stars |
|-----------|----------|-----------|
| Vertigo |1958 |8.4

---

## Query 49

**Question:** Write a SQL query to find the years when most of the ‘Mystery Movies’ produced. Count the number of generic title and compute their average rating. Group the result set on movie release year, generic title. Return movie year, generic title, number of generic title and average rating.

**SQL Query:**

```sql
SELECT 
    mv.mov_year,
    gn.gen_title,
    COUNT(mv.mov_id),
    AVG(mrt.rev_stars)
FROM movie mv
JOIN movie_genres mg 
    ON mv.mov_id = mg.mov_id
JOIN genres gn 
    ON mg.gen_id = gn.gen_id
LEFT JOIN movie_rating mrt 
    ON mv.mov_id = mrt.mov_id
WHERE gn.gen_title = 'Mystery'
GROUP BY 
    mv.mov_year,
    gn.gen_title;
```

**Output:**
| mov_year | gen_title | total_movies | avg_rating |
|----------|-----------|--------------|------------|
|1958| Mystery |1| 8.40000|
|1999| Mystery |1| null |
|1985| Mystery |1| null |

---

## Query 50

**Question:** Write a query in SQL to generate a report, which contain the fields movie title, name of the female actor, year of the movie, role, movie genres, the director, date of release, and rating of that movie.

**SQL Query:**

```sql

SELECT 
    mv.mov_title,
    ac.act_fname,
    ac.acrt_lname,
    mv.mov_year,
    mc.role,
    gn.gen_title,
    dr.dir_fname,
    dr.dir_lname,
    mv.mov_dt_rel,
    mrt.rev_stars 
FROM movie mv
JOIN movie_cast mc ON mv.mov_id = mc.mov_id
JOIN actor ac ON ac.act_id = mc.act_id
JOIN movie_genres mg ON mg.mov_id = mv.mov_id
JOIN genres gn ON gn.gen_id = mg.gen_id
JOIN movie_direction md ON md.mov_id = mv.mov_id
JOIN director dr ON dr.dir_id = md.dir_id
LEFT JOIN movie_rating mrt ON mrt.mov_id = mv.mov_id
WHERE ac.act_gender = 'F';
```

**Output:**
| mov_title | act_fname | acrt_lname | mov_year | role | gen_title | dir_fname | dir_lname | mov_dt_rel | rating |
|------------------|-----------|------------|----------|--------------|-----------|-----------|-----------|------------|--------|
| The Innocents | Deborah | Kerr | 1961 | Miss Giddens | Horror | Jack | Clayton | 1962-02-19 | 7.9 |
| Eyes Wide Shut | Nicole | Kidman | 1999 | Alice Harford| Mystery | Stanley | Kubrick | NULL | NULL |
| Princess Mononoke | Claire | Danes | 1997 | San | Animation | Hayao | Miyazaki | 2001-10-19 | 8.4 |
| Aliens | Sigourney | Weaver | 1986 | Ripley | Action | James | Cameron | 1986-08-29 | 8.4 |

---