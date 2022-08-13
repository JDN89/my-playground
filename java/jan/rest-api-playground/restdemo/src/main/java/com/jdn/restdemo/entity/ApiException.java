package com.jdn.restdemo.entity;

import org.springframework.http.HttpStatus;

import java.time.ZonedDateTime;

public class ApiException {
    private final HttpStatus status;
    // gives full visibility of exception to the client
    // Normally you don't expose this to the client, added as an example
    // private Throwable throwable;
    private final String message;
    private final ZonedDateTime timestamp;

    public ApiException(HttpStatus status, String message, ZonedDateTime timestamp) {
        this.status = status;
        this.message = message;
        this.timestamp = timestamp;
    }

    public HttpStatus getStatus() {
        return status;
    }

    public String getMessage() {
        return message;
    }

    public ZonedDateTime getTimestamp() {
        return timestamp;
    }

/*
    public Throwable getThrowable() {
        return throwable;
    }
*/
}
