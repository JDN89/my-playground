package com.jdn.restdemo.exception;

import com.jdn.restdemo.entity.ApiException;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.web.bind.annotation.ControllerAdvice;
import org.springframework.web.bind.annotation.ExceptionHandler;
import org.springframework.web.bind.annotation.ResponseStatus;
import org.springframework.web.context.request.WebRequest;
import org.springframework.web.servlet.mvc.method.annotation.ResponseEntityExceptionHandler;

import java.time.ZoneId;
import java.time.ZonedDateTime;

@ControllerAdvice
@ResponseStatus
public class RespsonseEntityExceptionHandler extends ResponseEntityExceptionHandler {
    @ExceptionHandler(PersonNotFoundException.class)
    protected ResponseEntity<ApiException> personNotFoundException(PersonNotFoundException exception, WebRequest request) {
        ApiException apiException = new ApiException(HttpStatus.NOT_FOUND, exception.getMessage(), ZonedDateTime.now(ZoneId.of("Z")));
        return ResponseEntity.status(HttpStatus.NOT_FOUND).body(apiException);
    }
}
