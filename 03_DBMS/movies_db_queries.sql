---------------------- BASIC QUERIES --------------------------------------------

-- 1. Write a SQL query to find the name and year of the movies. Return movie title, movie release year.
select mov_title,mov_year from movie;

-- 2. Write a SQL query to find when the movie 'American Beauty' released. Return movie release year.
select mov_year from movie where mov_title = 'American Beauty';

-- 3. Write a SQL query to find the movie that was released in 1999. Return movie title.
select mov_title from movie where mov_year = 1999;

-- 4. Write a SQL query to find those movies, which were released before 1998. Return movie title.
select mov_title from movie where mov_year < 1998;

-- 5. Write a SQL query to find the name of all reviewers and movies together in a single list.
select rev_name AS REVIEWR_MOVIE from MOVIE_REVIEWER UNION select mov_title from MOVIE;

-- 6. Write a SQL query to find all reviewers who have rated seven or more stars to their rating. Return reviewer name.
select DISTINCT mr.rev_name from MOVIE_REVIEWER mr JOIN movie_rating rt on mr.rev_id = rt.rev_id where rt.rev_stars>=7; 

-- 7. Write a SQL query to find the movies without any rating. Return movie title.
select mv.MOV_TITLE from MOVIE mv JOIN MOVIE_RATING mr on mv.mov_id = mr.mov_id where mr.NUM_O_RATINGS IS NULL;

-- 8. Write a SQL query to find the movies with ID 905 or 907 or 917. Return movie title.
select mov_title from movie mv where mv.MOV_ID = 905 OR mv.mov_ID = 907 or mv.mov_id = 917; 

-- 9. Write a SQL query to find the movie titles that contain the word 'Boogie Nights'. Sort the result-set in ascending order by movie year. Return movie ID, movie title and movie release year.
select mv.mov_id, mv.mov_title, mv.mov_year from movie mv where mv.mov_title LIKE '%Boogie Nights%' ORDER by MOV_YEAR 

-- 10. Write a SQL query to find those actors with the first name 'Woody' and the last name 'Allen'. Return actor ID.
select act_id from actor where act_fname = 'Woody' AND acrt_lname = 'Allen';

------------------------------------SUB QUERIES---------------------------------------

-- 11. Write a SQL query to find the actors who played a role in the movie 'Annie Hall'. Return all the fields of actor table.
select * from actor where act_id in 
(select act_id from movie_cast where mov_id in 
(select mov_id from movie where mov_title = 'Annie Hall'));

-- 12. Write a SQL query to find the director of a film that cast a role in 'Eyes Wide Shut'. Return director first name, last name.
select dir_fname , dir_lname from director where dir_id in 
(select dir_id from movie_direction where mov_id in
(select mov_id from movie where mov_title = 'Eyes Wide Shut'));

-- 13. Write a SQL query to find those movies that have been released in countries other than the United Kingdom. Return movie title, movie year, movie time, and date of release, releasing country.
select mov_title, mov_year, mov_time, mov_dt_rel, mov_rel_country from movie where mov_rel_country <> 'UK';

--14. Write a SQL query to find for movies whose reviewer is unknown. Return movie title, year, release date, director first name, last name, actor first name, last name.

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
        WHERE rev_name is null)
    );


-- 15. Write a SQL query to find those movies directed by the director whose first name is Woddy and last name is Allen. Return movie title.
select mov_title from movie where mov_id in (
    select mov_id from movie_direction where dir_id in(
        select dir_id from director where dir_fname = 'Woody' AND dir_lname = 'Allen'
    )
)

-- 16. Write a SQL query to determine those years in which there was at least one movie that received a rating of at least three stars. Sort the result-set in ascending order by movie year. Return movie year.
select mov_year from movie where mov_id in (
    select mov_id from movie_rating where rev_stars >= 3  
) order by mov_year;

-- 17. Write a SQL query to search for movies that do not have any ratings. Return movie title.
select mov_title from movie where mov_id in (
    select mov_id from movie_rating where rev_stars is null
);

--	18. Write a SQL query to find those reviewers who have not given a rating to certain films. Return reviewer name.
select rev_name from movie_reviewer where rev_id  not in(
    select rev_id from movie_rating
);

-- 19. Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Sort the result-set in ascending order by reviewer name, movie title, review Stars. Return reviewer name, movie title, review Stars.
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

-- 20. Write a SQL query to find movies that have been reviewed by a reviewer and received a rating. Group the result set on reviewer’s name, movie title. Return reviewer’s name, movie title.
SELECT DISTINCT
    (SELECT mr.rev_name
     FROM movie_reviewer mr
     WHERE mr.rev_id = r.rev_id) AS rev_name,

    (SELECT m.mov_title
     FROM movie m
     WHERE m.mov_id = r.mov_id) AS mov_title
FROM movie_rating r
WHERE r.rev_stars IS NOT NULL;


--21. Write a SQL query to find those movies, which have received highest number of stars. Group the result set on movie title and sorts the result-set in ascending order by movie title. Return movie title and maximum number of review stars.
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

--22. Write a SQL query to find all reviewers who rated the movie 'American Beauty'. Return reviewer name. 
select rev_name from movie_reviewer where rev_id in(
    select rev_id from movie_rating where mov_id in(
        select mov_id from movie where mov_title  = 'American Beauty'
    )
);

--23. Write a SQL query to find the movies that have not been reviewed by any reviewer body other than 'Paul Monks'. Return movie title.
select mov_title from movie where mov_id in (
    select mov_id from movie_rating where rev_id in (
        select rev_id from movie_reviewer where rev_name = 'Paul Monks'
    )
);

--	24. Write a SQL query to find the movies with the lowest ratings. Return reviewer name, movie title, and number of stars for those movies.
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

--25. Write a SQL query to find the movies directed by 'James Cameron'. Return movie title.
select mov_title from movie where mov_id in (
    select mov_id from movie_direction where dir_id in (
        select dir_id from director where dir_fname = 'James' AND dir_lname = 'Cameron'
    )
);

--26. Write a query in SQL to find the movies in which one or more actors appeared in more than one film.
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

------------------------------------------JOINS---------------------------------------------

--27. Write a SQL query to find all reviewers whose ratings contain a NULL value. Return reviewer name.
select rev_name from movie_reviewer mr join movie_rating mrt on mr.rev_id = mrt.rev_id where rev_stars is null;

--28. Write a SQL query to find out who was cast in the movie 'Annie Hall'. Return actor first name, last name and role.
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

--29. Write a SQL query to find the director who directed a movie that featured a role in 'Eyes Wide Shut'. Return director first name, last name and movie title.
select dr.dir_fname, dr.dir_lname, mv.mov_title from movie mv join movie_direction md 
    on mv.mov_id = md.mov_id join director dr on md.dir_id =dr.dir_id where mv.mov_title = 'Eyes Wide Shut'; 

--30. Write a SQL query to find the director of a movie that cast a role as Sean Maguire. Return director first name, last name and movie title.
select dr.dir_fname, dr.dir_lname, mv.mov_title from movie_direction md join movie mv 
on md.mov_id = mv.mov_id join director dr on md.dir_id = dr.dir_id join movie_cast mc 
on mv.mov_id = mc.mov_id where mc.role = 'Sean Maguire';

--31.Write a SQL query to find out which actors have not appeared in any movies between 1990 and 2000 (Begin and end values are included.). Return actor first name, last name, movie title and release year.

select ac.act_fname, ac.acrt_lname, mv.mov_title, mv.mov_year from movie mv join movie_cast mc
on mv.mov_id = mc.mov_id join actor ac on ac.act_id = mc.act_id where mv.mov_year between 1990 and 2000

--32.Write a SQL query to find the directors who have directed films in a variety of genres. Group the result set on director first name, last name and generic title. Sort the result-set in ascending order by director first name and last name. Return director first name, last name and number of genres movies.
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

--33. Write a SQL query to find the movies with year and genres. Return movie title, movie year and generic title.
select mv.mov_year , gn.gen_title from movie mv join movie_genres mg on 
mv.mov_id = mg.mov_id join genres gn on gn.gen_id = mg.gen_id

--34.  Write a SQL query to find all the movies with year, genres, and name of the director.
select mv.mov_year,gn.gen_title,dr.dir_fname from movie mv join movie_direction md 
on mv.mov_id = md.mov_id join director dr on md.dir_id = dr.dir_id join movie_genres mg on
mg.mov_id = mv.mov_id join genres gn on mg.gen_id = gn.gen_id;

--35. Write a SQL query to find the movies released before 1st January 1989. Sort the result-set in descending order by date of release. Return movie title, release year, date of release, duration, and first and last name of the director.
select mv.mov_title, mv.mov_year, mv.mov_dt_rel, mv.mov_time, dr.dir_fname,dr.dir_lname from
movie mv join movie_direction md on mv.mov_id = md.mov_id join director dr
on dr.dir_id = md.dir_id where mv.mov_dt_rel < DATE '1989-01-01' order by mv.mov_dt_rel desc;

--36. Write a SQL query to calculate the average movie length and count the number of movies in each genre. Return genre title, average time and number of movies for each genre.
select gn.gen_title, AVG(mv.mov_time), COUNT(mg.mov_id) from movie mv join movie_genres mg on 
mv.mov_id = mg.mov_id join genres gn on gn.gen_id = mg.gen_id group by gn.gen_title

-- 37. Write a SQL query to find movies with the shortest duration. Return movie title, movie year, director first name, last name, actor first name, last name and role.
SELECT mv.mov_title , mv.mov_year , dr.dir_fname,dr.dir_lname , ac.act_fname, ac.acrt_lname , mc.role
FROM 
movie mv JOIN movie_cast mc ON mc.mov_id = mv.mov_id
		JOIN actor ac ON ac.act_id = mc.act_id
        JOIN movie_direction md ON md.mov_id = mv.mov_id
        JOIN director dr ON dr.dir_id = md.dir_id
WHERE 
	mv.mov_time <= ALL(SELECT mov_time FROM movie);

-- 38. Write a SQL query to find the years in which a movie received a rating of 3 or 4. Sort the result in increasing order on movie year.
SELECT mv.mov_year  FROM
movie mv JOIN movie_rating mrt ON mv.mov_id = mrt.mov_id 
WHERE mrt.rev_stars= 3 OR mrt.rev_stars = 4 
GROUP BY mv.mov_year
ORDER BY mv.mov_year;

-- 39. Write a SQL query to get the reviewer name, movie title, and stars in an order that reviewer name will come first, then by movie title, and lastly by number of stars.
SELECT mr.rev_name  , mv.mov_title  , mrt.rev_stars
FROM 
movie mv JOIN movie_rating mrt ON mv.mov_id = mrt.mov_id
		JOIN movie_reviewer mr ON mr.rev_id = mrt.rev_id
		ORDER BY mr.rev_name ASC,
				 mv.mov_title ASC,
                 mrt.rev_stars ASC;

-- 40. Write a SQL query to find those movies that have at least one rating and received the most stars. Sort the result-set on movie title. Return movie title and maximum review stars.
SELECT 
    mv.mov_title,
    MAX(mrt.rev_stars) 
FROM movie mv
JOIN movie_rating mrt
    ON mv.mov_id = mrt.mov_id
GROUP BY mv.mov_title
HAVING MAX(mrt.rev_stars) = (
    SELECT MAX(rev_stars)
    FROM movie_rating
)
ORDER BY mv.mov_title;

-- 41. Write a SQL query to find out which movies have received ratings. Return movie title, director first name, director last name and review stars.
SELECT mv.mov_title, dr.dir_fname , dr.dir_lname, mrt.rev_stars AS review_stars
FROM 
movie mv JOIN movie_rating mrt ON mv.mov_id = mrt.mov_id
		JOIN movie_direction md ON mv.mov_id = md.mov_id
        JOIN director dr ON dr.dir_id = md.dir_id
WHERE 
	mrt.num_o_ratings > 0;
                 
-- 42.  Write a SQL query to find movies in which one or more actors have acted in more than one film. Return movie title, actor first and last name, and the role.
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

-- 43.  Write a SQL query to find the actor whose first name is 'Claire' and last name is 'Danes'. Return director first name, last name, movie title, actor first name and last name, role.
SELECT dr.dir_fname, dr.dir_lname,mv.mov_title , ac.act_fname , ac.acrt_lname , mc.role
FROM 
actor ac JOIN movie_cast mc ON mc.act_id = ac.act_id
		JOIN movie mv ON mv.mov_id = mc.mov_id
        JOIN movie_direction md on md.mov_id = mc.mov_id
        JOIN director dr ON dr.dir_id = md.dir_id
WHERE ac.act_fname = 'Claire' AND ac.acrt_lname='Danes';

-- 44.  Write a SQL query to find for actors whose films have been directed by them. Return actor first name, last name, movie title and role.
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
    
-- 45. Write a SQL query to find the cast list of the movie ‘Chinatown’. Return first name, last name.
SELECT ac.act_fname, ac.acrt_lname 
FROM 
actor ac JOIN movie_cast mc ON ac.act_id = mc.act_id
		JOIN movie mv ON mc.mov_id = mv.mov_id
WHERE mv.mov_title = 'Chinatown';
    
-- 46. Write a SQL query to find those movies where actor’s first name is 'Harrison' and last name is 'Ford'. Return movie title.
SELECT mv.mov_title
FROM 
movie mv JOIN movie_cast mc ON mv.mov_id = mc.mov_id
		JOIN actor ac ON ac.act_id = mc.act_id
WHERE 
	ac.act_fname='Harrison' AND ac.acrt_lname='Ford';
 
-- 47. Write a SQL query to find the highest-rated movies. Return movie title, movie year, review stars and releasing country.
SELECT 
    mv.mov_title,
    mv.mov_year,
    mrt.rev_stars,
    mv.mov_rel_country
FROM movie mv
JOIN movie_rating mrt ON mv.mov_id = mrt.mov_id

WHERE mrt.rev_stars = (select max(rev_stars) from movie_rating);

-- 48.  Write a SQL query to find the highest-rated ‘Mystery Movies’. Return the title, year, and rating.
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

-- 49. Write a SQL query to find the years when most of the ‘Mystery Movies’ produced. Count the number of generic title and compute their average rating. Group the result set on movie release year, generic title. Return movie year, generic title, number of generic title and average rating.
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
    
-- 50. Write a query in SQL to generate a report, which contain the fields movie title, name of the female actor, year of the movie, role, movie genres, the director, date of release, and rating of that movie.

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