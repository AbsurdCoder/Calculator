# Calculator

A simple digital Calculator with basic functionality :
Add
Subtract
Multiply
Divide

An implementation of Struct and Impl to deal with the Variables in more readable manner.


@AbsurdC

SELECT 
    app.tier1, 
    app.grn, 
    app.name, 
    inn.* 
FROM 
    dbo.AppInfo app
JOIN
    (SELECT 
         curr.appId, 
         curr.type, 
         curr.mnth AS current_month, 
         curr.cnt AS current_month_count,
         COALESCE(prev.cnt, 0) AS previous_month_count, 
         prev.mnth,
         CASE
             WHEN COALESCE(prev.cnt, 0) = 0 THEN NULL
             ELSE (curr.cnt - COALESCE(prev.cnt, 0))
         END AS month_over_month 
     FROM
         (SELECT 
              H.type,
              DATEPART(MONTH, H.executionTS) AS mnth, 
              B.appId, 
              COUNT(*) AS cnt 
          FROM 
              dbo.HealthChecks H 
          JOIN 
              (SELECT 
                   A.appId, 
                   MAX(A.runId) AS rid, 
                   DATEPART(MONTH, A.executionTS) AS mnth 
               FROM 
                   dbo.RunInfo A 
               WHERE 
                   A.appId IN (SELECT appId FROM dbo.AppInfo) 
                   AND A.executionTS > DATEADD(MONTH, -2, GETDATE()) 
               GROUP BY 
                   A.dayofmonth, A.appId) B 
          ON H.runId = B.rid 
          GROUP BY 
              H.type, B.mnth, B.appId) curr 
     LEFT JOIN 
         (SELECT 
              H.type,
              DATEPART(MONTH, H.executionTS) AS mnth, 
              B.appId, 
              COUNT(*) AS cnt 
          FROM 
              dbo.HealthChecks H 
          JOIN 
              (SELECT 
                   A.appId, 
                   MAX(A.runId) AS rid, 
                   DATEPART(MONTH, A.executionTS) AS mnth 
               FROM 
                   dbo.RunInfo A 
               WHERE 
                   A.appId IN (SELECT appId FROM dbo.AppInfo) 
                   AND A.executionTS BETWEEN DATEADD(MONTH, -14, GETDATE()) AND DATEADD(MONTH, -2, GETDATE())
               GROUP BY 
                   A.dayofmonth, A.appId) B 
          ON H.runId = B.rid 
          GROUP BY 
              H.type, B.mnth, B.appId) prev 
     ON curr.type = prev.type 
        AND curr.appId = prev.appId 
        AND (
            (curr.mnth = 1 AND prev.mnth = 12 AND YEAR(curr.executionTS) = YEAR(prev.executionTS) + 1) 
         OR (curr.mnth != 1 AND prev.mnth = curr.mnth - 1 AND YEAR(curr.executionTS) = YEAR(prev.executionTS))
        )
    ) inn 
ON app.appId = inn.appId
